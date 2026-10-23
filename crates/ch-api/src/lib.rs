use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use ch_auth::{authenticate, Principal};
use ch_config::RuntimeConfig;
use ch_errors::HorizonError;
use ch_evidence::EvidenceBatch;
use ch_orchestrator::{ExceptionRequest, HorizonService, RegisterCommitment};
use ch_repository::Repository;
use serde::Serialize;
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
#[derive(Clone)]
pub struct AppState {
    service: HorizonService,
    token: String,
}
#[derive(Serialize)]
struct Health {
    status: &'static str,
    service: &'static str,
}
fn correlation(headers: &HeaderMap) -> String {
    ch_middleware::validate_correlation(
        headers
            .get("x-correlation-id")
            .and_then(|v| v.to_str().ok()),
    )
    .unwrap_or_else(|_| "invalid-correlation".into())
}
fn failure(error: HorizonError, id: String) -> Response {
    let problem = error.problem(id);
    (
        StatusCode::from_u16(problem.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(problem),
    )
        .into_response()
}
async fn auth(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: axum::extract::Request,
    next: Next,
) -> Response {
    match authenticate(
        headers.get("authorization").and_then(|v| v.to_str().ok()),
        &state.token,
    ) {
        Ok(principal) => {
            request.extensions_mut().insert(principal);
            next.run(request).await
        }
        Err(error) => failure(error, correlation(&headers)),
    }
}
async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        service: "commitment-horizon",
    })
}
async fn create(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(command): Json<RegisterCommitment>,
) -> Response {
    if let Err(error) = ch_middleware::validate_idempotency(
        headers.get("idempotency-key").and_then(|v| v.to_str().ok()),
    ) {
        return failure(error, correlation(&headers));
    }
    match state.service.register(command, &correlation(&headers)) {
        Ok(record) => (StatusCode::CREATED, Json(record)).into_response(),
        Err(error) => failure(error, correlation(&headers)),
    }
}
async fn list(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    match state.service.list() {
        Ok(records) => Json(records).into_response(),
        Err(error) => failure(error, correlation(&headers)),
    }
}
async fn get_one(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    match state.service.get(&id) {
        Ok(record) => Json(record).into_response(),
        Err(error) => failure(error, correlation(&headers)),
    }
}
async fn evaluate(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(batch): Json<EvidenceBatch>,
) -> Response {
    match state.service.evaluate(batch, &correlation(&headers)) {
        Ok(result) => (StatusCode::CREATED, Json(result)).into_response(),
        Err(error) => failure(error, correlation(&headers)),
    }
}
async fn exception(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    axum::Extension(principal): axum::Extension<Principal>,
    Path(id): Path<String>,
    Json(request): Json<ExceptionRequest>,
) -> Response {
    match state
        .service
        .exception(&id, request, &principal.role, &correlation(&headers))
    {
        Ok(record) => (StatusCode::CREATED, Json(record)).into_response(),
        Err(error) => failure(error, correlation(&headers)),
    }
}
async fn audit(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    match state.service.audit(&id) {
        Ok(records) => Json(records).into_response(),
        Err(error) => failure(error, correlation(&headers)),
    }
}
pub fn router(config: &RuntimeConfig) -> Result<Router, HorizonError> {
    let repository = Repository::open(&config.namespace_dir())?;
    let state = Arc::new(AppState {
        service: HorizonService::new(repository)?,
        token: config.dev_token.clone(),
    });
    let protected = Router::new()
        .route("/commitments", post(create).get(list))
        .route("/commitments/{id}", get(get_one))
        .route("/commitments/{id}/evaluate", post(evaluate))
        .route("/commitments/{id}/exceptions", post(exception))
        .route("/commitments/{id}/audit", get(audit))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth));
    Ok(Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .nest("/api/v1", protected)
        .with_state(state)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()))
}
pub async fn serve(config: RuntimeConfig) -> Result<(), HorizonError> {
    let listener = tokio::net::TcpListener::bind(config.bind)
        .await
        .map_err(|e| HorizonError::Dependency(e.to_string()))?;
    axum::serve(listener, router(&config)?)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .map_err(|e| HorizonError::Dependency(e.to_string()))
}

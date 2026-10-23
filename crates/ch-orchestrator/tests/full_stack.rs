use ch_errors::HorizonError;
use ch_evidence::EvidenceBatch;
use ch_orchestrator::{ExceptionRequest, HorizonService, RegisterCommitment};
use ch_repository::Repository;
use ch_types::EvidencePoint;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn persists_real_composed_journey_and_denial_has_no_effect() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "commitment-horizon-integration-{}-{nonce}",
        std::process::id()
    ));
    let service =
        HorizonService::new(Repository::open(&root).expect("repository")).expect("service");

    service
        .register(
            RegisterCommitment {
                id: "checkout-availability".into(),
                name: "Checkout availability".into(),
                owner: "payments".into(),
                target: 0.95,
                window_minutes: 60,
            },
            "integration-register",
        )
        .expect("register");

    let result = service
        .evaluate(
            EvidenceBatch {
                commitment_id: "checkout-availability".into(),
                source: "synthetic-integration-v1".into(),
                points: [0.99, 0.96, 0.88, 0.81]
                    .into_iter()
                    .enumerate()
                    .map(|(index, observed)| EvidencePoint {
                        sequence: index as u64 + 1,
                        observed,
                        timestamp_ms: 1_700_000_000_000 + index as i64 * 60_000,
                    })
                    .collect(),
            },
            "integration-evaluate",
        )
        .expect("evaluate through calibration, e-process, RuVector, and RVF");
    assert_eq!(result.commitment_id.as_str(), "checkout-availability");
    assert_eq!(result.correlation_id, "integration-evaluate");

    let count_before_denial = service.audit("checkout-availability").expect("audit").len();
    let denial = service.exception(
        "checkout-availability",
        ExceptionRequest {
            reason: "maintenance window".into(),
            duration_minutes: 30,
        },
        "viewer",
        "integration-denied",
    );
    assert!(matches!(denial, Err(HorizonError::Forbidden(_))));
    assert_eq!(
        service.audit("checkout-availability").expect("audit").len(),
        count_before_denial,
        "Regorus denial must have no durable effect"
    );

    service
        .exception(
            "checkout-availability",
            ExceptionRequest {
                reason: "maintenance window".into(),
                duration_minutes: 30,
            },
            "operator",
            "integration-approved",
        )
        .expect("operator recovery");

    drop(service);
    let restarted = HorizonService::new(Repository::open(&root).expect("repository"))
        .expect("restart from RVF");
    assert!(
        restarted
            .audit("checkout-availability")
            .expect("audit")
            .len()
            > count_before_denial
    );
    drop(restarted);
    std::fs::remove_dir_all(&root).expect("remove isolated integration namespace");
}

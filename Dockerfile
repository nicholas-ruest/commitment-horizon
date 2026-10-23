FROM rust:1.99.0-bookworm AS backend
WORKDIR /src
COPY . .
RUN cargo build --locked --release -p ch-api
FROM debian:bookworm-slim AS runtime
RUN useradd -r -u 10001 horizon && mkdir -p /data && chown horizon /data
COPY --from=backend /src/target/release/commitment-horizon-api /usr/local/bin/
USER horizon
ENV HORIZON_BIND=0.0.0.0:8080 HORIZON_DATA_ROOT=/data HORIZON_NAMESPACE=cloud-run HORIZON_DEV_TOKEN=replace-at-runtime
EXPOSE 8080
ENTRYPOINT ["commitment-horizon-api"]

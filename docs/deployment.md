# Deployment and rollback

The committed manifest is Cloud Run-compatible but is not proof of deployment. Bind the runtime service account to only the authorized persistence interface and inject `HORIZON_DEV_TOKEN` from Secret Manager. The local RVF file proves the prototype path; production must mount/use the authorized RVF persistence service rather than Cloud Run ephemeral disk.

Deploy a source-SHA-tagged image, verify `/readyz`, then route traffic. Roll back with `gcloud run services update-traffic commitment-horizon-api --to-revisions PREVIOUS=100 --region REGION`. No production deployment was authorized for this build.

# Dioxus → Production Deployment TODO

## M1 — Podman locally

- [x] Install Podman
- [ ] Learn Podman images vs containers
- [ ] Create `Containerfile`
- [ ] Build Dioxus app image
- [ ] Run Dioxus container locally
- [ ] Configure port mapping
- [ ] Configure environment variables
- [ ] Check container logs
- [ ] Configure container restart policy

## M2 — PostgreSQL locally

- [ ] Run PostgreSQL with Podman
- [ ] Create persistent PostgreSQL volume
- [ ] Configure `DATABASE_URL`
- [ ] Connect Dioxus → PostgreSQL
- [ ] Run SQLx migrations
- [ ] Verify database survives container recreation
- [ ] Test PostgreSQL backup
- [ ] Test PostgreSQL restore

## M3 — Alpine containers

- [ ] Learn Alpine basics
- [ ] Create Alpine builder image
- [ ] Install Rust in builder
- [ ] Install Dioxus CLI in builder
- [ ] Build Dioxus app on Alpine
- [ ] Build for ARM64 / musl
- [ ] Create minimal Alpine runtime image
- [ ] Copy only runtime artifacts into final image
- [ ] Verify final image does not contain Rust/Cargo/Dioxus CLI
- [ ] Run Alpine image locally
- [ ] Test Dioxus + PostgreSQL using Alpine containers

## M4 — Oracle Cloud TEST VM

- [ ] Create Oracle ARM64 TEST VM
- [ ] Configure SSH access
- [ ] Configure firewall
- [ ] Install Podman
- [ ] Configure Podman
- [ ] Run PostgreSQL test container
- [ ] Deploy Dioxus container manually
- [ ] Configure environment variables
- [ ] Configure persistent volumes
- [ ] Verify application from Internet
- [ ] Verify PostgreSQL connectivity
- [ ] Verify container restart after reboot

## M5 — GitHub + self-hosted runner

- [ ] Create GitHub repository
- [ ] Configure private repository
- [ ] Create GitHub Actions workflow
- [ ] Install GitHub Actions Runner on TEST VM
- [ ] Register ARM64 self-hosted runner
- [ ] Verify GitHub → TEST VM communication
- [ ] Run a simple GitHub Actions job
- [ ] Verify job executes on TEST VM

## M6 — CI: Build + Test

- [ ] Checkout source code on TEST VM
- [ ] Install/build required dependencies
- [ ] Build Dioxus application
- [ ] Start test PostgreSQL
- [ ] Run SQLx migrations
- [ ] Run Rust tests
- [ ] Run application health check
- [ ] Build Podman image
- [ ] Start application container
- [ ] Run integration tests
- [ ] Fail CI when tests fail

## M7 — Container Registry

- [ ] Choose container registry
- [ ] Create registry repository
- [ ] Authenticate TEST VM to registry
- [ ] Push successful test image
- [ ] Use Git commit SHA as image tag
- [ ] Record immutable image digest
- [ ] Verify image can be pulled independently

## M8 — Oracle Cloud PROD VM

- [ ] Create Oracle ARM64 PROD VM
- [ ] Configure SSH access
- [ ] Configure firewall
- [ ] Install Podman
- [ ] Configure Podman
- [ ] Create production PostgreSQL container
- [ ] Create persistent PostgreSQL volume
- [ ] Configure production environment variables
- [ ] Configure secrets
- [ ] Manually pull tested image
- [ ] Manually run Dioxus container
- [ ] Run production migrations
- [ ] Verify application
- [ ] Verify PostgreSQL persistence
- [ ] Verify container restart after reboot

## M9 — Automated deployment

- [ ] Define TEST deployment workflow
- [ ] Build image on TEST VM
- [ ] Run tests on TEST VM
- [ ] Push image only after tests pass
- [ ] Deploy exact tested image to PROD
- [ ] Run production migrations
- [ ] Restart production container
- [ ] Run production health check
- [ ] Implement failed-deployment handling
- [ ] Keep previous production image available for rollback
- [ ] Test rollback

## M10 — Production backups

### PostgreSQL

- [ ] Create automated PostgreSQL backup
- [ ] Encrypt backups
- [ ] Store backups outside PROD VM
- [ ] Configure backup retention
- [ ] Verify backup files
- [ ] Test database restore
- [ ] Schedule periodic restore tests

### Source code

- [ ] Keep GitHub as primary source repository
- [ ] Create independent Git mirror
- [ ] Store Git mirror separately from PROD
- [ ] Verify repository can be restored

### Secrets/configuration

- [ ] Document required production secrets
- [ ] Store secrets securely
- [ ] Create recovery procedure
- [ ] Document production configuration

## M11 — Disaster recovery

- [ ] Document how to recreate TEST VM
- [ ] Document how to recreate PROD VM
- [ ] Document Podman setup
- [ ] Document database setup
- [ ] Document application deployment
- [ ] Document secret/configuration recovery
- [ ] Test recovery from a lost PROD VM
- [ ] Test recovery from a lost PostgreSQL database
- [ ] Test recovery from lost GitHub access
- [ ] Measure recovery time

## Final architecture

```text
                    GitHub
                 source + CI/CD
                      │
                      ▼
              TEST Oracle ARM64
             ┌─────────────────┐
             │ GitHub Runner   │
             │ Podman          │
             │ Build           │
             │ Test            │
             │ PostgreSQL test │
             └────────┬────────┘
                      │
                 tested image
                      │
                      ▼
              Container Registry
                      │
                      ▼
              PROD Oracle ARM64
             ┌─────────────────┐
             │ Podman          │
             │ Dioxus app      │
             │ PostgreSQL prod │
             └────────┬────────┘
                      │
                 DB backups
                      │
                      ▼
                Off-site storage
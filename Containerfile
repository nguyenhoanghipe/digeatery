# ============================================================
# Build environment
# ============================================================
FROM rust:slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

# ============================================================
# Dependency planning
# ============================================================
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ============================================================
# Application build
# ============================================================
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo check
RUN cargo test

# Install curl for cargo-binstall
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*

# Install Dioxus CLI
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:${PATH}"

# Build the Dioxus application
RUN dx bundle --web --release


# ============================================================
# Runtime
# ============================================================
FROM debian:stable-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/dx/digeatery/release/web/ /usr/local/app
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT ["/usr/local/app/server"]

#podman build -t digeatery:test .

#podman run --rm -it \
#  --name digeatery \
#  -p 8080:8080 \
#  -e DATABASE_URL='postgres://nguyenhoanghipe:H!3ppostgres@host.containers.internal:5432/digeatery' \
#  digeatery:test

#podman run --rm -it \
#  --name digeatery \
#  -p 8080:8080 \
#  --env-file .env.container \
#  digeatery:test



# attempt to use alpine

#FROM rust:alpine AS chef
#ENV CARGO_BUILD_JOBS=1
#RUN apk add --no-cache \
##    build-base \
#    pkgconf \
#    # needed for building dx \
#    openssl-dev \
#    # binstall script uses curl \
#    curl
#RUN wget -qO- https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh
#RUN cargo binstall cargo-chef --locked -y
#
#WORKDIR /app
#
#FROM chef AS planner
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM chef AS builder
#COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --release --recipe-path recipe.json
#COPY . .
#
#RUN cargo binstall dioxus-cli --locked -y
#RUN dx bundle --web --release
#
#FROM alpine:latest AS runtime
##RUN apk add --no-cache ca-certificates
#WORKDIR /usr/local/app
#COPY --from=builder /app/target/dx/digeatery/release/web/ ./
#
#ENV IP=0.0.0.0
#ENV PORT=8080
#EXPOSE 8080
#ENTRYPOINT [ "/usr/local/app/server" ]

# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY ../backend/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY ../backend/ .
RUN cargo build --target x86_64-unknown-linux-musl --bin dominux_drive_backend

FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/debug/dominux_drive_backend /usr/local/bin/
USER myuser
ENTRYPOINT ["/usr/local/bin/dominux_drive_backend"]

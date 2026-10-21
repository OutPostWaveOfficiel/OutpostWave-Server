# syntax=docker/dockerfile:1
# Build:  docker build -f docker/Dockerfile -t outpostwave-server .
# Builder and runtime share the same Debian release (glibc compatibility).

FROM rust:1.97-slim-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p ow-server

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /src/target/release/outpostwave-server /usr/local/bin/outpostwave-server
COPY config/config.toml /etc/outpostwave/config.toml
USER nonroot
EXPOSE 8080
VOLUME ["/var/lib/outpostwave"]
ENTRYPOINT ["/usr/local/bin/outpostwave-server", "--config", "/etc/outpostwave/config.toml"]

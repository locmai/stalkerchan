FROM rust:1.67.1-slim-bullseye AS build

WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./src ./src

# on rebuilds, we explicitly cache our rust build dependencies to speed things up
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    rustup install stable; \
    cargo build --workspace --release; \
    objcopy --compress-debug-sections target/release/stalkerchan ./stalkerchan

# stage two - we'll utilize a second container to run our built binary from our first container - slim containers!
FROM debian:11.6-slim as deploy

RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
    apt clean autoclean; \
    apt autoremove --yes; \
    rm -rf /var/lib/{apt,dpkg,cache,log}/;

WORKDIR /deploy

COPY --from=build /app/stalkerchan ./

CMD ["./stalkerchan"]

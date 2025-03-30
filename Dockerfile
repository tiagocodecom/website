FROM rustlang/rust:nightly-bullseye-slim AS builder

# Install Rust OS dependencies
RUN set -ex; \
    apt-get update; \
    apt-get install -y \
        build-essential \
        libssl-dev \
        pkg-config \
        openssl \
        ca-certificates \
        curl \
        bash \
        binaryen; \
    apt-get autoremove -y; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*

# Install Rust target
RUN set -ex; \
    rustup target add wasm32-unknown-unknown;

# Install project dependencies
RUN set -ex; \
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash; \
    cargo binstall -y wasm-bindgen-cli; \
    cargo binstall -y cargo-leptos;

WORKDIR /app

COPY . .

ENV LEPTOS_TAILWIND_VERSION="v3.4.16"

# Build the project
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bullseye-slim AS runner

WORKDIR /app

# Copy project builded files
COPY --from=builder /app/target/release/website /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
ENV LEPTOS_TAILWIND_VERSION="v3.4.16"

EXPOSE 8080

# Start the server
CMD ["/app/website"]


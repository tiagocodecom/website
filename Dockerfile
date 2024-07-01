ARG RUST_VERSION="1.79-alpine"

FROM rust:${RUST_VERSION} as development

LABEL author="Tiagocode <santiagomm1997@gmail.com>"
LABEL mantainer="Tiagocode <santiagomm1997@gmail.com>"
LABEL org.opencontainers.image.source="https://github.com/tiagocodecom/website"

WORKDIR /tmp/app

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN apk update && \
    apk add --no-cache musl-dev && \
    apk add --no-cache  \
        openssl  \
        openssh \
        curl  \
        bash  \
        git \
        zsh \
        npm \
        libc-dev \
        binaryen

RUN npm install -g sass

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall -y trunk
RUN cargo binstall -y cargo-leptos

COPY . .

EXPOSE 3000

CMD ["sh", "-c", "trunk serve --port 3000"]

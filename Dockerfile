FROM rust:1.64.0-alpine3.16 AS builder
WORKDIR /usr/src/discorss
COPY . .

RUN apk add --no-cache curl unzip musl-dev \ 
  && curl -fsSL -o cargo-make.zip \
  https://github.com/sagiegurari/cargo-make/releases/download/0.36.1/cargo-make-v0.36.1-x86_64-unknown-linux-musl.zip \
  && unzip cargo-make \
  && mv cargo-make-v0.36.1-x86_64-unknown-linux-musl/cargo-make . \
  && chmod +x cargo-make \
  && ./cargo-make make build-release

FROM gcr.io/distroless/static

COPY --from=builder /usr/src/discorss/target/x86_64-unknown-linux-musl/release/discorss .
CMD [ "/discorss" ]

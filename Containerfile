FROM docker.io/rust:alpine3.16 AS builder
WORKDIR /discorss

COPY . .
RUN apk add --no-cache curl unzip musl-dev
RUN curl -fsSL -o cargo-make.zip \
  https://github.com/sagiegurari/cargo-make/releases/download/0.36.2/cargo-make-v0.36.2-x86_64-unknown-linux-musl.zip \
  && unzip cargo-make \
  && mv cargo-make-v0.36.2-x86_64-unknown-linux-musl/cargo-make . \
  && chmod +x cargo-make 
RUN rustup toolchain install beta \
  && rustup default beta \
  RUN ./cargo-make make build-release

FROM gcr.io/distroless/static

COPY --from=builder /discorss/target/x86_64-unknown-linux-musl/release/discorss .
VOLUME [ "/data", "/logs" ]
CMD [ "/discorss" ]

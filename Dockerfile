FROM denoland/deno:alpine-1.25.4 AS builder
WORKDIR /discorss

COPY . .
RUN mkdir dist
RUN deno bundle src/index.ts -- dist/index.js

FROM denoland/deno:distroless-1.25.4
WORKDIR /discorss

COPY --from=builder /discorss/dist/index.js .
CMD [ "run", "-A", "index.js" ]

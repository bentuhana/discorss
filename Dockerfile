FROM denoland/deno:alpine-1.26.0 AS builder
WORKDIR /discorss

COPY . .
RUN mkdir dist
RUN deno bundle src/index.ts -- dist/index.js

FROM denoland/deno:alpine-1.26.0
WORKDIR /discorss

COPY --from=builder /discorss/dist/index.js .
VOLUME [ "/rss" ]
CMD [ "run", "-A", "index.js" ]

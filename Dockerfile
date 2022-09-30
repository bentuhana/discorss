FROM denoland/deno:alpine-1.26.0 AS builder
WORKDIR /discorss

COPY . .
RUN mkdir dist
RUN deno task bundle

FROM denoland/deno:alpine-1.26.0
WORKDIR /discorss
VOLUME [ "/discorss/rss" ]

COPY --from=builder /discorss/dist/index.js .
CMD [ "run", "-A", "index.js" ]

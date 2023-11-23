FROM rust:alpine3.18 AS builder
WORKDIR /workdir
COPY ./ ./
RUN apk add --update --no-cache musl-dev &&\
    cargo build --release

FROM scratch
COPY --from=builder /workdir/target/release/gpts-code-analyst /
COPY --from=builder /workdir/static /static
ENTRYPOINT ["/gpts-code-analyst"]

FROM ekidd/rust-musl-builder:stable AS builder
RUN cargo install svgbob_cli --version 0.6.6 --target x86_64-unknown-linux-musl

FROM alpine:3.15
COPY --from=builder /home/rust/.cargo/bin/svgbob_cli /usr/local/bin

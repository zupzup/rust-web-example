FROM ekidd/rust-musl-builder:nightly as builder
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/analyzer /usr/local/bin/analyzer
EXPOSE 8080
CMD ["/usr/local/bin/analyzer"]

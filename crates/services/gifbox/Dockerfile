# Build Stage
FROM ghcr.io/voxly-gg/base:latest AS builder

# Bundle Stage
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /home/rust/src/target/release/voxly-gifbox ./

EXPOSE 14706
USER nonroot
CMD ["./voxly-gifbox"]

FROM rust:1.80-slim as build

WORKDIR /work
COPY . .
RUN cargo build -p stateroom-cli --release

FROM debian:bookworm-slim

COPY --from=build /work/target/release/stateroom /stateroom
ENTRYPOINT [ "/stateroom" ]
CMD ["serve", "/dist"]

FROM rust:latest as build

WORKDIR /work
COPY . .
RUN cargo build -p stateroom-cli --release

FROM gcr.io/distroless/cc-debian11

COPY --from=build /work/target/release/stateroom /stateroom
ENTRYPOINT [ "/stateroom" ]
CMD ["serve", "/dist"]

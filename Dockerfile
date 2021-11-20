FROM rust:latest as build

WORKDIR /work
COPY . .
RUN cargo build -p jamsocket-cli --release

FROM gcr.io/distroless/cc-debian11

COPY --from=build /work/target/release/jamsocket /jamsocket
ENTRYPOINT [ "/jamsocket" ]
CMD ["serve", "/dist"]

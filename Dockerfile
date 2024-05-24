FROM rust:1.78.0-slim-bullseye AS build

WORKDIR /usr/src/generic-merge

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /usr/src/generic-merge/target/release/generic-merge /
CMD [ "./generic-merge" ]
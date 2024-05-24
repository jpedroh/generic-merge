FROM rust:1.78.0-slim-bullseye AS build

WORKDIR /usr/src/generic-merge

COPY . . 
RUN cargo build --locked --release

FROM alpine:3.14
COPY --from=build /usr/src/generic-merge/target/release/* /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/generic-merge" ]

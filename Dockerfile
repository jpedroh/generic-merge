FROM rust:1.78.0-slim-bullseye AS build

WORKDIR /usr/src/generic-merge

COPY . . 
RUN cargo build --locked --release

CMD ["./target/release/generic-merge"]
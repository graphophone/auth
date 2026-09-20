FROM rust:1.97-alpine3.23 AS build
WORKDIR /app

RUN apk add protoc

RUN mkdir src
RUN echo "fn main() { println!(\"Hello world\"); }" >> src/main.rs
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build

COPY . .
RUN cargo build

FROM scratch

COPY --from=build /app/target/debug/auth .
COPY config config

EXPOSE 8080

CMD ["./auth"]

FROM rust:1.97-alpine3.23 AS build
WORKDIR /app

RUN apk add protoc

COPY . .

RUN cargo build

FROM scratch

COPY --from=build /app/target/debug/auth .
COPY config config

EXPOSE 8080

CMD ["./auth"]

FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path ./api_server


FROM rust:1.76-slim

ENV DATABASE_URL mysql://${USER}:${PASSWORD}@db:${MYSQL_PORT}/${MYSQL_DATABASE}
ENV TZ=Asia/Tokyo
ENV PORT=80

RUN apt-get update && apt-get install -y \
    libmariadb-dev-compat \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/api_server /usr/local/bin/api_server

ENTRYPOINT ["api_server"]

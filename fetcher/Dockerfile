FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path ./fetcher --bin fetcher


FROM rust:1.79-slim

ENV DATABASE_URL mysql://${USER}:${PASSWORD}@db:${MYSQL_PORT}/${MYSQL_DATABASE}
ENV TZ=Asia/Tokyo

RUN apt-get update && apt-get install -y \
    libmariadb-dev-compat \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/fetcher /usr/local/bin/fetcher

ENTRYPOINT ["fetcher"]

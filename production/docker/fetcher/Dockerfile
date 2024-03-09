FROM rust:1.76 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path . --bin fetcher


FROM rust:1.76-slim

ENV DATABASE_URL mysql://${USER}:${PASSWORD}@db:${MYSQL_PORT}/${MYSQL_DATABASE}
ENV TZ=Asia/Tokyo

RUN apt-get update && apt-get install -y \
    libmariadb-dev-compat \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/fetcher /usr/local/bin/fetcher

CMD ["fetcher"]

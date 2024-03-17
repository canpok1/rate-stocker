FROM rust:latest
WORKDIR /workspace
COPY . .
RUN cargo install diesel_cli

WORKDIR /workspace/migrator
ENV DATABASE_URL mysql://${USER}:${PASSWORD}@db:${MYSQL_PORT}/${MYSQL_DATABASE}
ENTRYPOINT ["diesel"]
CMD ["migration", "run"]

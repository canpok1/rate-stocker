FROM rust:latest

RUN cargo install \
    diesel_cli \
    cargo-make \
    && apt update \
    && apt install -y \
    jq \
    openjdk-17-jre \
    maven \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p ~/bin/openapitools \
    && curl https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh > /usr/local/bin/openapi-generator-cli \
    && chmod u+x /usr/local/bin/openapi-generator-cli 

ENV PATH $PATH:~/bin/openapitools/

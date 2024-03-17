FROM rust:latest

RUN cargo install diesel_cli \
    && apt update \
    && apt install -y \
        jq \
        openjdk-17-jre \
        maven \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p ~/bin/openapitools \
    && curl https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh > ~/bin/openapitools/openapi-generator-cli \
    && chmod u+x ~/bin/openapitools/openapi-generator-cli \
    && ~/bin/openapitools/openapi-generator-cli version

ENV PATH $PATH:~/bin/openapitools/

FROM rust:1.79.0

WORKDIR /code

COPY Cargo.lock /code/Cargo.lock
COPY Cargo.toml /code/Cargo.toml

COPY src /code/src

RUN \
    cargo build --release && \
    mv target/*/polyglot-bot polyglot-bot && \
    rm -rf target

FROM rust:1.79.0

COPY --from=0 /code/polyglot-bot /polyglot-bot

CMD ["/polyglot-bot"]

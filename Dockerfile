FROM rust

WORKDIR /usr/src/myapp

COPY config  /usr/local/cargo
COPY log.txt ./
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY database ./database
COPY server ./server

RUN cargo build --release

CMD cargo run --release

EXPOSE 9902
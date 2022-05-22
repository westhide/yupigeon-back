FROM rust

WORKDIR /usr/src/myapp

COPY config  /usr/local/cargo
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY macro-lib ./macro-lib
COPY database ./database
COPY mongo ./mongo
COPY server ./server

RUN cargo build --release

CMD cargo run --release

EXPOSE 9901
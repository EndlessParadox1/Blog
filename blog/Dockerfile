FROM rust:latest

WORKDIR /root/server

COPY . .

RUN cargo build --release

EXPOSE 2396

CMD ./target/release/blog >log.out 2>&1

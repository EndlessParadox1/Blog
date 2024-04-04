FROM rust:latest

WORKDIR /root/server

COPY . .

EXPOSE 2396

RUN cargo build --release

CMD ["./target/release/blog"] >log.out 2>&1

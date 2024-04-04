FROM rust:lastest

WORKDIR /root/server

COPY . .

RUN cargo build --release

CMD ["./target/release/blog"] >log.out 2>&1

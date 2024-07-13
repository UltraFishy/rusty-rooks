FROM rust:latest

WORKDIR /home/ultrafish/projects/winter_hack_2024/rusty-rooks

COPY . .

RUN cargo build --release 

ENTRYPOINT ["./target/release/server"]
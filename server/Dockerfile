FROM rust:latest

# Set the working directory for common
WORKDIR /home/ultrafish/projects/winter_hack_2024/rusty-rooks/server

COPY . . 

RUN cargo clean

# Build the server application
RUN cargo build --release

# Set the entrypoint
ENTRYPOINT ["./target/release/server"]

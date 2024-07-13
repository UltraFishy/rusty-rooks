FROM rust:latest

# Set the working directory
WORKDIR /home/ultrafish/projects/winter_hack_2024/rusty-rooks/client

# Copy the source code
COPY . .

# Build the client application
RUN cargo build --release

# Set the entrypoint
ENTRYPOINT ["./target/release/client"]

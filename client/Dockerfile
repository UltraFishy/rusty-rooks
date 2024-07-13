FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Build the client application
RUN cargo build --release

# Set the entrypoint
ENTRYPOINT ["./target/release/client"]

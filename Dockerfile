# Use the latest official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the entire project into the container
COPY . .

# Install necessary dependencies for building the project
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Build the project in release mode
RUN cargo build --release

# Set the command to run the application
CMD ["cargo", "run"]

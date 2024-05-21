# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/online_llm

# Copy the Rocket.toml file
COPY Rocket.toml ./

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Set Rocket environment to production
ENV ROCKET_ENV=production

# Install dependencies and build the application
RUN cargo build --release

# Expose the port that Rocket will run on
EXPOSE 8000

# Set the entry point for the container
CMD ["./target/release/online_llm"]

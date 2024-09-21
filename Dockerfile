# Stage 1: Build the application
FROM rust:1.81 as builder
RUN apt update && apt install -y curl gnupg2

RUN curl -s https://packagecloud.io/install/repositories/ookla/speedtest-cli/script.deb.sh | bash
RUN apt install -y speedtest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to build dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Build dependencies
RUN cargo build --release

# Now copy the actual source code
COPY src src

# Build the actual application
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/bin/speedtest /usr/bin/speedtest
COPY --from=builder /usr/src/app/target/release/prometheus-speedtest-exporter /usr/local/bin/prometheus-speedtest-exporter

EXPOSE 9516
CMD ["/usr/local/bin/prometheus-speedtest-exporter"]

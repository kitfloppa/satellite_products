FROM rust:1.74-slim-bookworm as builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config cmake g++ && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/zonaris
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/zonaris /usr/local/bin/zonaris
CMD ["zonaris"]

FROM rust:1.80-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/abyssal-nexus /usr/local/bin/
EXPOSE 3000
CMD ["abyssal-nexus"]

# docker build -t abyssal-nexus .
# docker run -p 3000:3000 abyssal-nexus
FROM rust:latest AS builder

WORKDIR /opt/did-api

COPY src /opt/did-api/src

COPY Cargo.toml /opt/did-api
COPY Cargo.lock /opt/did-api

RUN cargo build --release

FROM debian:latest
RUN apt-get update && apt-get install -y --no-install-recommends \
		ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /opt/did-api

COPY --from=builder \
    /opt/did-api/target/release/did-api \
    ./

RUN chmod +X did-api

ENTRYPOINT ["/opt/did-api/did-api"]

# docker build -t did-api .
# docker run -d -p 8080:8080 did-api
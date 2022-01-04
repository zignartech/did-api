FROM rust:latest

WORKDIR /opt/api-did

COPY --chown=root:root src /opt/api-did/src
COPY --chown=root:root accounts_stronghold /opt/api-did
COPY Cargo.toml /opt/api-did

RUN cargo build --release

WORKDIR /opt/api-did/target/release/

RUN chmod +X api-did

COPY development.env /opt/api-did/target/release/

ENTRYPOINT ["/opt/api-did/target/release/api-did"]

# docker build -t api-did-parse .
# docker run -p 6000:6000 api-did-parse
FROM rust:1.49.0-buster

RUN apt-get update
RUN apt-get -y install pkg-config libssl-dev

WORKDIR /app
COPY tests tests
COPY solutions solutions
RUN find tests -name Cargo.toml -exec cargo fetch --manifest-path {} \;
RUN find /usr/local/cargo/registry/src -type f -name '*.rs' -exec chmod 644 {} \;
RUN rm -rf solutions

COPY entrypoint.sh ./

ENTRYPOINT ["/app/entrypoint.sh"]

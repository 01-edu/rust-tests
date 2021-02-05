FROM rust:1.48.0-buster

RUN apt-get update
RUN apt-get -y install pkg-config libssl-dev

WORKDIR /app
COPY . .
RUN find tests -name Cargo.toml -exec cargo fetch --manifest-path {} \;
RUN rm -rf solutions

ENTRYPOINT ["/app/entrypoint.sh"]

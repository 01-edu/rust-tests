FROM rust:1.48.0-buster

WORKDIR /app

COPY . .
RUN find tests -name Cargo.toml -exec cargo fetch --manifest-path {} \;
RUN rm -rf solutions

ENTRYPOINT ["/app/entrypoint.sh"]

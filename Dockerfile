### Test Image specific config

FROM rust:1.85-slim-bookworm

RUN apt-get update
RUN apt-get -y install pkg-config libssl-dev moreutils

WORKDIR /app
COPY tests tests
COPY tests_utility tests_utility
COPY solutions solutions
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN parallel cargo fetch --manifest-path -- $(find tests -name Cargo.toml)
RUN find /usr/local/cargo/registry/src -type f -name '*.rs' -exec chmod 644 {} \;
RUN rm -rf solutions

### Default configs
# ℹ️ URL of the Repository
LABEL org.opencontainers.image.source=https://github.com/01-edu/rust-tests
# ℹ️ Description of the Test Image
LABEL org.opencontainers.image.description="01 Edu - Rust Test Image"
# ℹ️ Licence type – MIT by default
LABEL org.opencontainers.image.licenses=MIT

COPY entrypoint.sh ./

ENTRYPOINT ["/app/entrypoint.sh"]

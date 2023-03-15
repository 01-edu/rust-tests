FROM rust:1.68-slim-buster

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

COPY entrypoint.sh ./

ENTRYPOINT ["/app/entrypoint.sh"]

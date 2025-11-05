### Test Image specific config
FROM rustlang/rust:nightly-alpine

# Install required packages
RUN apk add --no-cache \
    bash \
    coreutils \
    findutils \
    gawk \
    grep \
    sed \
    parallel \
    pkgconfig \
    openssl-dev \
    build-base \
    moreutils

WORKDIR /app

# Copy project structure
COPY tests tests
COPY tests_utility tests_utility
COPY solutions solutions

# Use sparse index for Cargo (faster network)
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Pre-fetch all dependencies
RUN find tests -name Cargo.toml | parallel cargo fetch --manifest-path {}

# Fix permissions for Rust crates (avoid “permission denied”)
RUN find /usr/local/cargo/registry/src -type f -name '*.rs' -exec chmod 644 {} \;

# Remove solutions (only student code will be mounted)
RUN rm -rf solutions

### Metadata labels
LABEL org.opencontainers.image.source="https://github.com/01-edu/rust-tests"
LABEL org.opencontainers.image.description="01 Edu - Rust Test Image (Alpine)"
LABEL org.opencontainers.image.licenses="MIT"

# Copy entrypoint and isolate script
COPY entrypoint.sh ./
COPY isolate.sh ./

RUN chmod +x ./entrypoint.sh ./isolate.sh

ENTRYPOINT ["/app/entrypoint.sh"]

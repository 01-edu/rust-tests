FROM alpine/git:1.0.20 as cloner

ADD https://time.is /tmp/invalidate_cache
RUN git clone --single-branch --branch rust-piscine https://github.com/01-edu/public.git

COPY .ssh /root/.ssh
RUN chmod 400 /root/.ssh/id_ed25519
RUN git clone git@github.com:01-edu/rust-piscine-solutions.git
RUN rm -rf /root/.ssh

FROM rust:1.48.0-buster

WORKDIR /app

COPY --from=cloner /git/public public
COPY --from=cloner /git/rust-piscine-solutions rust-piscine-solutions
RUN find public/rust/tests -name Cargo.toml -exec cargo fetch --manifest-path {} \;
RUN rm -rf rust-piscine-solutions

COPY entrypoint.sh .
ENTRYPOINT ["/app/entrypoint.sh"]

FROM rust:1.62.0 as builder
WORKDIR /usr/src/yeenserve
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo fetch
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/yeenserve /usr/local/bin/yeenserve

ENV ROCKET_ADDRESS="0.0.0.0"
ENV YEENSERVE_PATH="/pics"
CMD ["yeenserve"]

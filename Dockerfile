# --- Build ---
FROM rust:1-slim-trixie AS build
WORKDIR /src

# Eerst alleen de manifesten, zodat dependencies gecachet worden tussen builds.
COPY Cargo.toml Cargo.lock ./
COPY bot/Cargo.toml bot/Cargo.toml
RUN mkdir -p bot/src && echo "fn main() {}" > bot/src/main.rs \
    && cargo build --release -p broodkast \
    && rm -rf bot/src

COPY bot bot
# Zorg dat cargo de echte bron als nieuwer ziet dan de dummy.
RUN touch bot/src/main.rs && cargo build --release -p broodkast

# --- Runtime ---
FROM debian:trixie-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home broodkast \
    && mkdir -p /data && chown broodkast /data

COPY --from=build /src/target/release/broodkast /usr/local/bin/broodkast

USER broodkast
# Mount hier een volume (op Railway: /data), anders ben je de database kwijt bij elke deploy.
ENV DATABASE_URL=sqlite:///data/broodkast.db
EXPOSE 8080
CMD ["broodkast"]

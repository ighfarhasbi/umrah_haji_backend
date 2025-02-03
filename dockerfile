FROM rust

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock .env ./
COPY src ./src

RUN cargo fetch

COPY . .

CMD ["cargo", "run", "--release"]

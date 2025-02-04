FROM rust AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock .env ./
COPY src ./src

RUN cargo fetch

COPY . .

# Ekspos port aplikasi (ubah sesuai kebutuhan)
EXPOSE 3000

CMD ["cargo", "run", "--release"]
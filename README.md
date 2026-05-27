# axum-mongodb-starter
![Rust](https://img.shields.io/badge/Rust-1.87-brightgreen.svg?style=flat-square)
![Axum](https://img.shields.io/badge/Axum-0.7-brightgreen.svg?style=flat-square)
![MongoDB](https://img.shields.io/badge/MongoDB-latest-brightgreen.svg?style=flat-square)

## Introduction
A minimal backend starter for building APIs with [Axum](https://github.com/tokio-rs/axum), [mongodb driver](https://www.mongodb.com/docs/drivers/rust/), and [MongoDB](https://www.mongodb.com/).

## Project structure
```
src/
├── config.rs       # Environment and app configuration
├── models/         # MongoDB document structs
├── schemas/        # Request/response DTOs
├── handlers/       # Axum route handlers
└── main.rs         # App entrypoint
tests/              # Integration tests (require a running server)
```

## Prerequisites
- Rust 1.87+
- make
- A MongoDB instance (local or [MongoDB Atlas](https://www.mongodb.com/atlas))

## Installation
1. Copy and fill in the environment file:
   ```sh
   cp env.sample .env
   ```
   Set `MONGO_URI`, `MONGO_DB`, `BACKEND_NAME`, and `BACKEND_VERSION`.

2. Fetch dependencies:
   ```sh
   make install
   ```

## Getting Started

Available make commands:
```sh
make help       # list all commands
make install    # fetch Cargo dependencies
make dev        # run development server
make test       # run integration tests (requires running server)
make build      # build Docker image
make run        # run app via Docker
make mongo      # start a local MongoDB container
make mongo-stop # stop the local MongoDB container
make clean      # remove build artifacts
```

### Run locally
```sh
make mongo   # if you need a local MongoDB
make dev
```
API available at `http://localhost:8000`.

### Run with Docker
```sh
make build
make run
```

## Tests
Integration tests — require the app to be running first:
```sh
make test
```

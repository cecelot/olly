# olly

Olly is a game client and server for the two-player strategy board game, [Othello](https://en.wikipedia.org/wiki/Reversi).

## Develop

This repository consists of a Rust web server using [axum](https://docs.rs/axum/latest/axum/), and a [Solid](https://solidjs.com) client (in `client/`).

It is recommended to use the Nix development shell at the root of this repository to automatically install all necessary dependencies (excluding Docker, which must be installed manually).

### Environment Variables

- `DATABASE_URL` (default: `postgres://olly:password@0.0.0.0:5432/olly`) - specifies the address of the PostgreSQL database

### Steps

1. Start the PostgreSQL database: `docker compose up`
2. Migrate the database: `cargo run --package migration up`
3. Start the `axum` server: `cargo run`
4. Run the web client: `cd client && npm run dev`

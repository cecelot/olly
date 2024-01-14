# othello

An implementation of the two-player strategy board game, Othello. This is a variant of [Reversi](https://en.wikipedia.org/wiki/Reversi) with a fixed initial setup of the board.

This repository consists of a Rust web server using [axum](https://docs.rs/axum/latest/axum/), and a [Solid](https://solidjs.com) client (in `client/`).

## Develop

It is recommended to use the Nix development shell at the root of this repository to automatically install all necessary dependencies (excluding Docker, which must be installed manually).

1. Start the PostgreSQL database: `docker compose up`
2. Migrate the database: `cargo run --package migration up`
3. Start the `axum` server: `cargo run`
4. Run the web client: `cd client && npm run dev`

# olly

Olly is a game client and server for the two-player strategy board game, [Othello](https://en.wikipedia.org/wiki/Reversi).

# Features

- Play Othello with friends by inviting via username (no matchmaking)
- User registration and account (username/password) management
- Send and receive friend requests from others
- View your pending (incoming and outgoing) invites to games as well as currently active games
- Abandon games at any point before a player wins
- Request (classical AI) moves generated using [Negamax](https://en.wikipedia.org/wiki/Negamax) algorithm (as an API endpoint: `/companion`)

# Develop

This repository consists of a Rust web server using [axum](https://docs.rs/axum/latest/axum/), and a [Next.js](https://nextjs.org) client (in `client/`).

It is recommended to use the Nix development shell at the root of this repository to automatically install all necessary dependencies (excluding Docker, which must be installed manually).

## Steps

1. Start the background services (server, PostgreSQL database, Redis cache): `docker compose up -d`
2. Migrate the database: `cargo run --package migration up`
3. Run the web client: `cd client && npm run dev`

## Testing

**Frontend:** Use `cypress` e2e test runner (`npm run test`). After each subsequent execution, `sea-orm-cli migrate fresh` must be run to ensure that app state is refreshed to defaults. Otherwise, some tests may fail.

**Backend:** Use Cargo's built in runner (`cargo test`). After each subsequent execution, `sea-orm-cli migrate fresh` must be run to ensure that app state is refreshed to defaults. Otherwise, some tests may fail.

## Environment Variables

- `DATABASE_URL` (default: `postgres://olly:password@db:5432/olly`) - specifies the address of the PostgreSQL database
- `REDIS_URL` (default: `redis://cache`) - specifies the address of the Redis server

# License

[MIT](https://github.com/cecelot/olly/blob/main/LICENSE)

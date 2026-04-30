# Obligatory assignment 3 - DATA2410
The assignment was completed in Rust using a postgres database.

## Quickstart
Prerequisites:
- Docker
- Rust-tools (cargo, rustc, usually installed with rustup or a different manager)

Steps to run:
1. `cp .env.template .env`
2. `docker compose up -d`
3. `cargo run`
4. Use browser/postman/http-yac to send requests(we supplied some in test.http at repo root for convenience)

## A more detailed explanation
1. DB:
  - Either use the included compose.yaml (with `docker compose up`) or the nix module (with `nix run`)
  - The database can be seeded using the POST requests written in plaintext in [test.http](./test.http).
  - The migration file is stored in `migrations/1_student.sql`. If the database is hosted with the nix module, this is automatically applied on startup. Otherwise, `sqlx`(sql lib) will regardless run unapplied migration when the project starts up.
  - The connection url must be set in env var DATABASE_URL(see `.env.template`) or the code will panic.
2. Project:
  - `cargo run`
3. Using:
  - There is no data in the DB, but some helpful POST requests are included in the repo. We run them with the [http-yac vscode extension](https://marketplace.visualstudio.com/items?itemName=anweber.vscode-httpyac).
  - The basic endpoints (from the C# template) are set up and working. Endpoint 6 and 7 (the task) are marked in the code by doc comments (`///`).

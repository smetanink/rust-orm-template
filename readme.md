# RUST template with ORM (PostgreSQL)
## Setup local environment
### Setup ENVs
```bash
cp ./.env.example ./.env
```
### Run PostgreSQL
```bash
docker compose up -d postgres
```
### Run migration
```bash
cargo run --bin migration up
```
### Check migration
```bash
cargo run
```
## Delete default migrations and write new
### Delete odd entities
```bash
cargo run --bin entities erase
```
### Create new migration
!!!Important. Due to SEA ORM error you must generate new migration first. Then delete old migrations.
!!!Other direction may corrupt your `src/migrator/mod.rs` file.

To create new migration run:
```bash
cargo run --bin new_migration migration_name
```
### Delete odd migrations
- Delete old migration files from `src/migrator`
- Delete old imported migration modules in `mod.rs`
### Run migration
```bash
cargo run --bin refresh
```
### Create new entities
```bash
cargo run --bin entities generate
```
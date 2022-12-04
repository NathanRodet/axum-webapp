# axum-webapp

In depth learning of Axum and Rust

## Setup db

### Login

- password = password
- user = postgres
- database = postgres

### Init and checkup

```
docker compose up
docker compose down

docker compose exec database /bin/bash
psql -U postgres -d postgres
\dt
\dt+
```

### sea-orm

```
cargo install sea-orm-cli
sea-orm-cli generate entity -o src/database
```

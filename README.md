# Auth

Testing out simple Rust web-framework by creating an authentication service using Axum and Postgres.

## Setup

1. To begin, fill the necessary environment variables in the `.env` file.
2. Run the Postgres database locally using Docker:

```bash
docker run --name postgres -e POSTGRES_PASSWORD=[your password] -p 5432:5432 -d postgres
```

3. To perform the migration, install `sqlx-cli`:

```bash
cargo install sqlx-cli
```

4. Run the migration:

```bash
sqlx migrate run
```

5. Run the server:

```bash
cargo run
```

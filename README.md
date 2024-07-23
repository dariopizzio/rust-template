# rust-template

Rust template for REST APIs using Axum and Diesel

## How to install Postgres & DieselCli (Brew)

```bash
brew install libpq
brew install postgresql
cargo install diesel_cli --no-default-features --features postgres
```

## Diesel Setup

```bash
diesel setup
diesel migration generate --diff-schema items
```

### Diesel useful commands

```bash
diesel migration run
diesel migration redo
```

## How to run the docker containers

```bash
docker-compose up
```

## How to run the application

If you have cargo-watch installed:

```bash
cargo watch -x run 
```

If not

```bash
cargo run
```

## Curl Example

curl -i --location 'localhost:8080/item/1'

## Troubleshooting

- If you get this error `ld: library 'pq' not found` when running `cargo run`, and Postgres and Diesel are already installed, run the following commands:

```bash
cargo clean
cargo build
```

## Missing features/improvements

- Tests/Code Documentation
- Graphql (Optional)
- Infrastructure for AWS using Terraform

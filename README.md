# rust-template
Rust template for REST APIs

# Brew install (Check)
brew install libpq
brew install postgresql
cargo install diesel_cli --no-default-features --features postgres

## Diesel Setup
```bash
diesel setup
diesel migration generate --diff-schema items
```

Useful commands:
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

curl -i --location 'localhost:8080/item/1'

curl --location 'localhost:8080/item' \
--header 'Content-Type: application/json' \
--data '{"item_name": "item"}'

------------------

Missing features:
Tracing
CQRS
Middleware authentication
Infra -> terraform
Rust macros
Add postman collection
Tests
Graphql

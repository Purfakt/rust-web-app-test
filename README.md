# Test project in Rust

This is a test project in Rust to try some existing Rust frameworks.

## Backend

The app backend is made with actix-web and expects a running instance of MongoDB on the default port (`27017`)

### Run

To run the backend:

```bash
docker compose up -d
cargo run --package backend
```

### Endpoints

See [test_api.rest](backend/test_api.rest)

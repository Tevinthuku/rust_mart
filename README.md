## Modular RustMart

### How to run

1. Start the DB `docker-compose up -d`
2. Set `DATABASE_URL` to your terminal env.

- `export DATABASE_URL=postgres://postgres:postgres@localhost/rust_mart_modular_monolith`

3. Run the `http_server`.
   - `cd http_server`
   - `cargo run`

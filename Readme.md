## item axum api

# Build the project

cargo run build

# Prepare the sqlx

// cargo sqlx prepare
// cargo sqlx prepare --database-url <DATABASE_URL> [-- <ARGS>...]

// cargo sqlx prepare --database-url DATABASE_URL

# cargo run with watch

// run rust program on watch use => cargo watch -q -c -w src/ -x run

// RUST_BACKTRACE=1 cargo run
// RUST_BACKTRACE=1 cargo watch -q -c -w src/ -x run

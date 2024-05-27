# pg_pgbouncer

Running PgBouncer directly from your `psql` shell.

## How to compile

Install the Rust toolchain
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install `cargo pgrx`
```
cargo install cargo-pgrx --locked --version '=0.11.4' --force
cargo pgrx init
```

Add extension to shared_preload_libraries:
```
echo "shared_preload_libraries = 'pg_pgbouncer'" >> ~/.pgrx/data-16/postgresql.conf
```

Build the extension and run it:
```
cargo pgrx run # opens psql automatically
```

Create the extension:
```
DROP EXTENSION IF EXISTS pg_pgbouncer; CREATE EXTENSION pg_pgbouncer;
```

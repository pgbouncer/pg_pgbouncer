# pg_pgbouncer

Running PgBouncer directly from your `psql` shell.

## How to compile

### Install the Rust toolchain
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install `cargo pgrx`
```
cargo install cargo-pgrx --locked --version '=0.11.4' --force
cargo pgrx init
```

### Add extension to shared_preload_libraries and build & install the extension
You can either choose to let pgrx to create a new database cluster for you and install pg_pgbouncer
extension there, or you can choose to install pg_pgbouncer extension to your existing database cluster.

1. Let pgrx to create a new database cluster:

    ```
    echo "shared_preload_libraries = 'pg_pgbouncer'" >> ~/.pgrx/data-16/postgresql.conf
    ```

    ```
    # builds & installs pg_pgbouncer.so into ~/.pgrx/16.0 and opens psql automatically for pg_pgbouncer database
    cargo pgrx run
    ```

    Create the extension in the psql session opened automatically:
    ```
    DROP EXTENSION IF EXISTS pg_pgbouncer; CREATE EXTENSION pg_pgbouncer;
    ```

2. Install pg_pgbouncer extension to your existing database cluster:

    ```
    echo "shared_preload_libraries = 'pg_pgbouncer'" >> /path/to/desired/postgresql.conf
    ```

    ```
    # builds & installs pg_pgbouncer.so using pg_config from PATH
    cargo pgrx install
    ```

    Create a database named as `pg_pgbouncer`:
    ```
    CREATE DATABASE pg_pgouncer;
    ```

    Connect to the database `pg_pgbouncer` and create the extension there:
    ```
    DROP EXTENSION IF EXISTS pg_pgbouncer; CREATE EXTENSION pg_pgbouncer;
    ```

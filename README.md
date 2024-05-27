# pg_pgbouncer

**DISCLAIMER: This is alpha quality software at the moment. It is not
recommended for production use (yet). Some API design decisions might
significantly change too.**

## How to compile and run

### Install the Rust toolchain
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Configure Rust for more speedy compilation (optional)

See [`CONTRIBUTING.md`][`https://github.com/pgbouncer/pg_pgbouncer/blob/main/README.md`] for details

### Install `cargo pgrx`
```
cargo install cargo-pgrx --locked --version '=0.11.4' --force
cargo pgrx init
```

### Add extension to shared_preload_libraries and build & install the extension
You can either choose to let pgrx to create a new database cluster for you and install pg_pgbouncer
extension there, or you can choose to install pg_pgbouncer extension to your existing database cluster.

1. Use pgrx-managed postgres server

   Configure postgres:

    ```
    echo "shared_preload_libraries = 'pg_pgbouncer'" >> ~/.pgrx/data-16/postgresql.conf
    echo "pg_pgbouncer.database = 'pg_pgbouncer'" >> ~/.pgrx/data-16/postgresql.conf
    ```

    Build & install pg_pgbouncer.so into the pgrx-managed postgres server:

    ```
    # builds & installs pg_pgbouncer.so into ~/.pgrx/16.0 and opens psql automatically for pg_pgbouncer database
    cargo pgrx run
    ```

    Create the extension in the psql session opened automatically:
    ```
    DROP EXTENSION IF EXISTS pg_pgbouncer; CREATE EXTENSION pg_pgbouncer;
    ```

2. Use pre-installed postgres server

    Configure postgres:
    ```
    echo "shared_preload_libraries = 'pg_pgbouncer'" >> /path/to/desired/postgresql.conf
    ```

    Build & install pg_pgbouncer.so into the postgres your effective pg_config utility points to:
    ```
    # builds & installs pg_pgbouncer.so using pg_config from PATH
    cargo pgrx install
    ```

    Configure the database name in which you will create the pg_pgbouncer extension:
    ```
    ALTER SYSTEM SET pg_pgbouncer.database='<dbname>';
    ```

    Restart postgres and connect to it using a client utility such as psql.

    Connect to the database pg_pgbouncer.database and create the extension there:
    ```
    DROP EXTENSION IF EXISTS pg_pgbouncer; CREATE EXTENSION pg_pgbouncer;
    ```

Check the postgres server log (for pgrx-managed Postgres the log is at
`~/.pgrx/16.log`). If you see the below line, pg_pgbouncer is working.
```
LOG:  Hello from inside the PgBouncer Manager BGWorker!
```

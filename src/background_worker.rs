use crate::gucs::PG_PGBOUNCER_DATABASE;
use crate::*;

#[pg_guard]
pub extern "C" fn _PG_init() {
    gucs::init();
    BackgroundWorkerBuilder::new("PgBouncer Manager")
        .set_function("pgbouncer_manager_main")
        .set_library("pg_pgbouncer")
        .set_restart_time(Some(Duration::from_secs(1)))
        .set_start_time(BgWorkerStartTime::ConsistentState)
        .enable_spi_access()
        .load();
}

#[pg_guard]
#[no_mangle]
pub extern "C" fn pgbouncer_manager_main() {
    // If pg_pgbouncer.database GUC is not set to a value,
    // pg_pgbouncer background worker will return 0 and thus be unregistered.
    let database_name = match PG_PGBOUNCER_DATABASE.get() {
        Some(s) => {
            log!("PG_PGBOUNCER_DATABASE is set to {}", s.to_str().unwrap());
            s.to_str().unwrap()
        }
        None => {
            log!("pg_pgbouncer.database is not defined. pg_pgbouncer background worker exits.");
            return;
        }
    };

    // these are the signals we want to receive.  If we don't attach the SIGTERM handler, then
    // we'll never be able to exit via an external notification
    BackgroundWorker::attach_signal_handlers(
        SignalWakeFlags::SIGHUP
            | SignalWakeFlags::SIGTERM
            | SignalWakeFlags::SIGINT
            | SignalWakeFlags::SIGCHLD,
    );

    // we want to be able to use SPI against the specified database (pg_pgbouncer), as the superuser which
    // did the initdb. You can specify a specific user with Some("my_user")
    BackgroundWorker::connect_worker_to_spi(Some(database_name), None);

    log!(
        "Hello from inside the {} BGWorker!",
        BackgroundWorker::get_name(),
    );

    create_dir_all(TEMP_DIR).unwrap();
    let mut state = ManagerState::default();

    // wake up every 10s or if we received a signal
    while BackgroundWorker::wait_latch(Some(Duration::from_secs(10))) {
        if let Err(err) = state.do_main_loop() {
            warning!("error in main pg_pgbouncer loop: {err}");
            // We're in an unknown state because of the error. Clear out previous_groups to
            // force reinitialization in the next loop.
            state.previous_groups = None;
        } else {
            state.previous_groups = Some(state.groups.clone());
        }
    }

    state
        .shutdown()
        .unwrap_or_else(|e| warning!("could not stop pgbouncers: {e}"));

    log!(
        "Goodbye from inside the {} BGWorker! ",
        BackgroundWorker::get_name()
    );
}

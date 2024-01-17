use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};
use std::ffi::CStr;

pub(crate) static PG_PGBOUNCER_DATABASE: GucSetting<Option<&'static CStr>> =
    GucSetting::<Option<&'static CStr>>::new(None);

pub fn init() {
    GucRegistry::define_string_guc(
        "pg_pgbouncer.database",
        "The database that the pg_pgbouncer BGWorker will connect to",
        "This should be the database that you ran `CREATE EXTENSION pg_pgbouncer` in",
        &PG_PGBOUNCER_DATABASE,
        GucContext::Postmaster,
        GucFlags::SUPERUSER_ONLY,
    );
}

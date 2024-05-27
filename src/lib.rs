use anyhow::{bail, Context, Result};
use defaultmap::DefaultBTreeMap;
use indoc::indoc;
use indoc::writedoc;
use nix::errno::Errno;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use pgrx::bgworkers::*;
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::{self, create_dir_all};
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::str::FromStr;
use std::time::Duration;
use std::time::SystemTime;
use tempfile::NamedTempFile;

mod auth_line;
mod background_worker;
mod child;
mod database;
mod group;
mod hba;
mod helpers;
mod manager;
mod peer;
mod pgbouncer;
mod settings;
mod snapshot;
mod udfs;
mod user;

use auth_line::*;
use child::*;
use database::*;
use group::*;
use hba::*;
use helpers::*;
use manager::*;
use peer::*;
use pgbouncer::*;
use settings::*;
use snapshot::*;
use user::*;

pgrx::pg_module_magic!();

extension_sql_file!("../bootstrap.sql", bootstrap);
extension_sql_file!("../pg_pgbouncer.sql");

const VERSION: &str = env!("CARGO_PKG_VERSION");
static BASE_DIR: &str = "pgbouncer";
static TEMP_DIR: &str = "pgbouncer/.tmp";

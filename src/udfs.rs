use crate::Group;
use anyhow::{Context, Result};
use pgrx::prelude::*;

#[pg_schema]
mod pgbouncer {
    use super::*;

    #[pg_extern]
    fn admin(command: &str, name: default!(&str, "'default'")) -> Result<()> {
        Group::all()?
            .values()
            .find(|g| g.name == name)
            .context("group not found")?
            .admin(command)?;
        Ok(())
    }
}

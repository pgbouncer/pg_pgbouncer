use crate::background_worker::RECONFIGURE;
use crate::pg_sys::ConditionVariableCancelSleep;
use crate::pg_sys::ConditionVariableTimedSleep;
use crate::Group;
use anyhow::{Context, Result};
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use pgrx::prelude::*;
use std::borrow::BorrowMut;
use std::time;

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

    #[pg_extern]
    fn reconfigure() -> i64 {
        let ten_millis = time::Duration::from_millis(10000);

        let curr_config_count = RECONFIGURE.share().count;

        let mut next_config_count = curr_config_count;

        signal::kill(Pid::from_raw(RECONFIGURE.share().pid), Signal::SIGHUP).unwrap();
        log!("Sent SIGHUP");

        while curr_config_count >= next_config_count {
            //thread::sleep(ten_millis);
            log!(
                "curr_config_count { } next_config_count { } ",
                curr_config_count,
                next_config_count
            );
            log!("Sleep");

            unsafe {
                if ConditionVariableTimedSleep(
                    RECONFIGURE.exclusive().cv.borrow_mut(),
                    ten_millis.as_millis().try_into().unwrap(),
                    pg_sys::WL_LATCH_SET | pg_sys::WL_TIMEOUT | pg_sys::WL_POSTMASTER_DEATH,
                ) {
                    log!("ConditionVariableTimedSleep timed out");
                } else {
                    log!("CV signalled");
                }
            }

            pg_sys::check_for_interrupts!();

            next_config_count = RECONFIGURE.share().count;
        }

        unsafe {
            ConditionVariableCancelSleep();
        }

        next_config_count as i64
    }
}

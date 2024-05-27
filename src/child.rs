use crate::*;

pub struct PgBouncerChild {
    pub process: PgBouncer,
    child: Option<Child>,
    started_at: Option<SystemTime>,
}

impl PgBouncerChild {
    pub fn new(process: PgBouncer) -> PgBouncerChild {
        PgBouncerChild {
            process,
            child: None,
            started_at: None,
        }
    }

    fn pid(&mut self) -> Result<Option<Pid>> {
        if let Some(ref mut child) = &mut self.child {
            if let Some(result) = child.try_wait()? {
                log!("PgBouncer child exited with {result}");
                self.child = None;
                return Ok(None);
            };
            return Ok(Some(Pid::from_raw(child.id().try_into()?)));
        }

        self.process.pid()
    }

    pub fn stop(&mut self) -> Result<()> {
        let Some(pid) = self.pid()? else {
            return Ok(());
        };

        signal::kill(pid, Signal::SIGTERM)?;
        if let Some(ref mut child) = &mut self.child {
            let result = child.wait()?;
            log!("PgBouncer child exited with {result}");
        } else {
            while self.pid()?.is_some() {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        self.child = None;
        self.started_at = None;
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        if self.pid()?.is_some() {
            return Ok(());
        };

        if !self.allowed_to_start() {
            warning!("PgBouncer exited very quickly, waiting a bit longer until restarting it");
            return Ok(());
        }
        log!("Starting PgBouncer process");
        self.started_at = Some(SystemTime::now());
        let ini = self.process.ini()?;
        let ini_str = ini.clone().into_os_string();
        let mut command = Command::new("pgbouncer");
        command.args([ini_str]);

        // Make sure SIGINT on manager is not forwarded to pgbouncer
        command.process_group(0);
        log!("{command:?}");
        let child = command.spawn()?;
        self.child = Some(child);
        Ok(())
    }

    pub fn reload(&mut self) -> Result<()> {
        let Some(pid) = self.pid()? else {
            return Ok(());
        };

        signal::kill(pid, Signal::SIGHUP)?;
        Ok(())
    }

    fn allowed_to_start(&self) -> bool {
        let Some(started_at) = self.started_at else {
            return true;
        };
        started_at + Duration::from_secs(10) < SystemTime::now()
    }
}

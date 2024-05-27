use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PgBouncer {
    pub group_id: i32,
    pub index: usize,
    pub port: i32,
    pub peer_offset: usize,
}

impl PgBouncer {
    pub fn new(group: Group, index: usize) -> PgBouncer {
        PgBouncer {
            group_id: group.id,
            index,
            port: 6432,
            peer_offset: group.peer_offset,
        }
    }

    pub fn pid(&self) -> Result<Option<Pid>> {
        // If there's no socket file it's not running
        if !self.socket()?.exists() {
            return Ok(None);
        }

        let pid_string = fs::read_to_string(self.pidfile()?)?;
        let pid = Pid::from_raw(i32::from_str(pid_string.trim())?);

        // Double check that the pid is correct
        let kill_result = signal::kill(pid, None);
        if let Err(Errno::ESRCH) = kill_result {
            return Ok(None);
        };

        kill_result?; // fail on any other errors that meant we could not signal
        Ok(Some(pid))
    }

    pub fn create_config(&self) -> Result<()> {
        let dir = self.dir()?;
        create_dir_all(&dir)?;
        let ini = self.ini()?;
        log!(
            "Writing config for PgBouncer peer {} in group {}, path {}",
            self.index,
            self.group_id,
            ini.to_string_lossy(),
        );
        let peers_ini = self.peers_ini()?;
        let databases_ini = self.databases_ini()?;
        let settings_ini = self.settings_ini()?;
        let users_ini = self.users_ini()?;

        let dir_str = dir.to_str().context("no utf8")?;
        let peers_ini_str = peers_ini.to_str().context("no utf8")?;
        let databases_ini_str = databases_ini.to_str().context("no utf8")?;
        let users_ini_str = users_ini.to_str().context("no utf8")?;
        let settings_ini_str = settings_ini.to_str().context("no utf8")?;
        let peer_id = self.peer_id();
        // Atomically write to the file
        let mut f = NamedTempFile::new_in(TEMP_DIR)?;
        writedoc!(
            f,
            r#"
        [peers]
        %include {peers_ini_str}

        [databases]
        %include {databases_ini_str}

        [users]
        %include {users_ini_str}

        [pgbouncer]
        %include {settings_ini_str}

        unix_socket_dir = {dir_str}
        peer_id = {peer_id}

        pidfile = {dir_str}/pgbouncer.pid
        "#,
        )?;
        f.persist(&ini)?;

        Ok(())
    }

    fn shared_dir(&self) -> Result<PathBuf> {
        Ok(std::env::current_dir()?
            .join(BASE_DIR)
            .join(self.group_id.to_string()))
    }

    pub fn dir(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join(self.index.to_string()))
    }

    pub fn ini(&self) -> Result<PathBuf> {
        Ok(self.dir()?.join("config.ini"))
    }

    pub fn peers_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("peers.ini"))
    }

    pub fn databases_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("databases.ini"))
    }

    pub fn users_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("users.ini"))
    }

    pub fn settings_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("settings.ini"))
    }

    pub fn pidfile(&self) -> Result<PathBuf> {
        Ok(self.dir()?.join("pgbouncer.pid"))
    }

    pub fn socket(&self) -> Result<PathBuf> {
        Ok(self.dir()?.join(format!(".s.PGSQL.{}", self.port)))
    }

    pub fn peer_id(&self) -> usize {
        self.index + self.peer_offset
    }

    pub fn admin(&self, command: &str) -> Result<()> {
        let dir = self.dir()?;
        let dir_str = dir.to_str().context("no utf8")?;
        let mut client = postgres::Config::new()
            .dbname("pgbouncer")
            .host(dir_str)
            .port(6432)
            .connect(postgres::NoTls)?;
        client.simple_query(command)?;
        Ok(())
    }
}

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
struct EphemeralState {
    restart_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroupState {
    version: String,
    group: Group,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub num_processes: usize,
    pub peer_offset: usize,
    pub stay_alive: bool,
    pub restart_target: i64,
    pub databases: Vec<Database>,
    pub users: Vec<User>,
    pub settings: Settings,
    pub hba_rules: Vec<HbaRule>,
    pub auth_lines: Vec<AuthLine>,
    pub peers: Vec<Peer>,
    pub processes: Vec<PgBouncer>,
}

impl Group {
    pub fn all() -> Result<BTreeMap<i32, Group>> {
        Snapshot::fetch()?.try_into()
    }

    fn shared_dir(&self) -> Result<PathBuf> {
        Ok(std::env::current_dir()?
            .join(BASE_DIR)
            .join(self.id.to_string()))
    }

    fn peers_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("peers.ini"))
    }

    fn databases_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("databases.ini"))
    }

    fn users_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("users.ini"))
    }

    fn settings_ini(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("settings.ini"))
    }

    fn hba_conf(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("hba.conf"))
    }

    fn auth_file_txt(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("auth_file.txt"))
    }

    fn state_file(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("state.json"))
    }

    fn ephemeral_state_file(&self) -> Result<PathBuf> {
        Ok(self.shared_dir()?.join("ephemeral_state.json"))
    }

    fn processes(&self) -> Result<Vec<PgBouncer>> {
        let mut processes = Vec::new();
        for i in 1..=self.num_processes {
            processes.push(PgBouncer::new(self.clone(), i));
        }
        Ok(processes)
    }

    pub fn update_children(&self, children: &mut Vec<PgBouncerChild>) -> Result<()> {
        if children.len() > self.num_processes {
            for child in children[self.num_processes..].iter_mut() {
                child.stop()?;
            }

            children.truncate(self.num_processes);
        }

        for i in children.len()..self.num_processes {
            children.push(PgBouncerChild::new(PgBouncer::new(self.clone(), i + 1)))
        }

        for child in children {
            child.process.peer_offset = self.peer_offset;
        }
        Ok(())
    }

    pub fn create_shared_configs(&self) -> Result<()> {
        let shared_dir = self.shared_dir()?;
        create_dir_all(shared_dir)?;

        let mut f = NamedTempFile::new_in(TEMP_DIR)?;
        for peer in self.peers.iter() {
            writeln!(f, "{peer}",)?;
        }
        for proc in self.processes.iter() {
            let dir = proc.dir()?;
            let dir_str: &str = dir.to_str().context("non utf8")?;
            writeln!(
                f,
                "{} = host={} port={}",
                proc.peer_id(),
                dir_str,
                proc.port
            )?;
        }
        let peers_ini = self.peers_ini()?;
        f.persist(peers_ini)?;

        let mut f = NamedTempFile::new_in(TEMP_DIR)?;
        for db in &self.databases {
            writeln!(f, "{db}")?;
        }
        let databases_ini = self.databases_ini()?;
        f.persist(databases_ini)?;

        // users
        let mut f = NamedTempFile::new_in(TEMP_DIR)?;
        for user in &self.users {
            writeln!(f, "{user}")?;
        }
        let users_ini = self.users_ini()?;
        f.persist(users_ini)?;

        let mut f = NamedTempFile::new_in(TEMP_DIR)?;
        for rule in &self.hba_rules {
            writeln!(f, "{rule}")?;
        }
        let hba_conf = self.hba_conf()?;
        f.persist(&hba_conf)?;

        let mut f = NamedTempFile::new_in(TEMP_DIR)?;
        for line in &self.auth_lines {
            writeln!(f, "{line}")?;
        }
        let auth_file_txt = self.auth_file_txt()?;
        f.persist(&auth_file_txt)?;

        let hba_conf_str = hba_conf.to_str().context("no utf8")?;
        let auth_file_txt_str = auth_file_txt.to_str().context("no utf8")?;

        // settings
        let mut f = NamedTempFile::new_in(TEMP_DIR)?;
        writedoc!(
            f,
            r#"
            {}
            auth_file = {auth_file_txt_str}
            auth_hba_file = {hba_conf_str}
            so_reuseport = 1
            "#,
            self.settings,
        )?;
        let settings_ini = self.settings_ini()?;
        f.persist(settings_ini)?;

        Ok(())
    }

    pub fn cleanup_old_files(&self) -> Result<()> {
        for dir in fs::read_dir(self.shared_dir()?)? {
            let dir = dir?;
            let path = dir.path();

            // skip files, these are the shared configs
            if !path.is_dir() {
                continue;
            }

            let process_index_opt = dir
                .file_name()
                .to_str()
                .and_then(|s| s.parse::<usize>().ok());
            // if the directory is not a number, then it's not a process directory, so we can delete it
            let Some(process_index) = process_index_opt else {
                log!("Removing directory {}", dir.path().to_string_lossy());
                fs::remove_dir_all(dir.path())?;
                continue;
            };

            if process_index > self.num_processes {
                log!("Removing directory {}", dir.path().to_string_lossy());
                fs::remove_dir_all(dir.path())?;
            }
        }
        Ok(())
    }

    pub fn write_state(&self) -> Result<()> {
        let state = GroupState {
            version: VERSION.to_string(),
            group: self.clone(),
        };
        let state_file = self.state_file()?;
        let f = NamedTempFile::new_in(TEMP_DIR)?;
        serde_json::to_writer(&f, &state)?;
        f.persist(state_file)?;
        Ok(())
    }

    // We're going to need this for letting pgbouncer stay alive across restarts, but currently
    // it's dead code.
    #[allow(dead_code)]
    fn read_state(&self) -> Result<Option<Group>> {
        let state_file = self.state_file()?;
        if !state_file.exists() {
            return Ok(None);
        }
        let state: GroupState = serde_json::from_reader(fs::File::open(state_file)?)?;
        if state.version != VERSION {
            return Ok(None);
        }
        Ok(Some(state.group))
    }

    pub fn changed(&self, state: &ManagerState) -> bool {
        let Some(ref previous_groups) = state.previous_groups else {
            return true;
        };
        previous_groups.get(&self.id) != Some(self)
    }

    pub fn update_restart_count(&self) -> Result<()> {
        let ephemeral_state_file = self.ephemeral_state_file()?;
        let restart_count = self.restart_target;
        let state = EphemeralState { restart_count };
        let f = NamedTempFile::new_in(TEMP_DIR)?;
        serde_json::to_writer(&f, &state)?;
        f.persist(ephemeral_state_file)?;
        Ok(())
    }

    fn current_restart_count(&self) -> Result<i64> {
        let ephemeral_state_file = self.ephemeral_state_file()?;
        if !ephemeral_state_file.exists() {
            return Ok(0);
        }
        let state: EphemeralState = serde_json::from_reader(fs::File::open(ephemeral_state_file)?)?;
        Ok(state.restart_count)
    }

    pub fn should_restart(&self) -> Result<bool> {
        Ok(self.current_restart_count()? != self.restart_target)
    }

    pub fn admin(&self, command: &str) -> Result<()> {
        let mut had_error = false;
        for process in self.processes()?.iter() {
            let result = process.admin(command);
            if let Err(error) = result {
                warning!(
                    "Error running admin command on process {}/{}: {}",
                    self.name,
                    process.index,
                    error
                );
                had_error = true;
            }
        }
        if had_error {
            bail!("Error running admin command, see warning(s) for details");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GroupRow {
    pub id: i32,
    pub name: String,
    pub num_processes: Option<i32>,
    pub peer_offset: i32,
    pub stay_alive: bool,
    pub restart_target: i64,
}

impl GroupRow {
    pub fn all() -> Result<BTreeMap<i32, GroupRow>> {
        Spi::connect(|client| {
            let tuple_table = client.select(
                indoc!(
                    "SELECT *
                    FROM pgbouncer.groups
                    ORDER BY id;"
                ),
                None,
                None,
            )?;
            let mut rows = BTreeMap::new();
            for tuple in tuple_table {
                let row = GroupRow {
                    id: tuple
                        .get_datum_by_name("id")?
                        .value::<i32>()?
                        .context("no id")?,
                    name: tuple
                        .get_datum_by_name("name")?
                        .value::<String>()?
                        .context("no name")?,
                    num_processes: tuple.get_datum_by_name("num_processes")?.value::<i32>()?,
                    peer_offset: tuple
                        .get_datum_by_name("peer_offset")?
                        .value::<i32>()?
                        .context("no peer_offset")?,
                    stay_alive: tuple
                        .get_datum_by_name("stay_alive")?
                        .value::<bool>()?
                        .context("no stay_alive")?,
                    restart_target: tuple
                        .get_datum_by_name("restart_target")?
                        .value::<i64>()?
                        .context("no restart_target")?,
                };
                log!("from bgworker: {:?}", &row);
                rows.insert(row.id, row);
            }
            Ok(rows)
        })
    }
}

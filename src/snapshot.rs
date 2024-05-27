use crate::*;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Snapshot {
    groups: BTreeMap<i32, GroupRow>,
    databases: BTreeMap<i32, Vec<Database>>,
    users: BTreeMap<i32, Vec<User>>,
    peers: BTreeMap<i32, Vec<Peer>>,
    settings: BTreeMap<i32, Settings>,
    hba_rules: BTreeMap<i32, Vec<HbaRule>>,
    auth_lines: BTreeMap<i32, Vec<AuthLine>>,
}

impl Snapshot {
    pub fn fetch() -> Result<Snapshot> {
        Ok(Snapshot {
            groups: GroupRow::all()?,
            databases: Database::all()?,
            users: User::all()?,
            peers: Peer::all()?,
            settings: Settings::all()?,
            hba_rules: HbaRule::all()?,
            auth_lines: AuthLine::all()?,
        })
    }
}

impl TryFrom<Snapshot> for BTreeMap<i32, Group> {
    type Error = anyhow::Error;
    fn try_from(mut snapshot: Snapshot) -> Result<Self> {
        let mut groups = BTreeMap::new();
        for group_row in snapshot.groups.values() {
            let num_processes = if let Some(num_processes) = group_row.num_processes {
                num_processes.try_into()?
            } else {
                num_cpus::get_physical()
            };
            let databases_opt: Option<Vec<Database>> = snapshot.databases.remove(&group_row.id);

            let Some(mut databases) = databases_opt else {
                warning!("group {} has no databases", &group_row.name);
                continue;
            };

            for database in databases.iter_mut() {
                if database.host.is_none() {
                    let host = postgres_socket_dirs()?;
                    database.host = Some(host);
                }
                if database.port.is_none() {
                    database.port = Some(postgres_port());
                }
            }

            let users = snapshot.users.remove(&group_row.id).unwrap_or_default();

            let peers = snapshot.peers.remove(&group_row.id).unwrap_or_default();

            let mut settings = snapshot.settings.remove(&group_row.id).unwrap_or_default();

            let hba_rules = snapshot.hba_rules.remove(&group_row.id).unwrap_or_default();

            if !hba_rules.is_empty() && settings.auth_type.is_none() {
                settings.auth_type = Some("hba".to_string());
            }

            let auth_lines = snapshot
                .auth_lines
                .remove(&group_row.id)
                .unwrap_or_default();

            let peer_offset: usize = group_row.peer_offset.try_into()?;

            let group = Group {
                id: group_row.id,
                name: group_row.name.clone(),
                num_processes,
                peer_offset,
                stay_alive: group_row.stay_alive,
                restart_target: group_row.restart_target,
                databases,
                users,
                settings,
                hba_rules,
                auth_lines,
                peers,
                processes: (1..=num_processes)
                    .map(|i| PgBouncer {
                        group_id: group_row.id,
                        index: i,
                        port: 6432,
                        peer_offset,
                    })
                    .collect(),
            };
            groups.insert(group.id, group);
        }
        Ok(groups)
    }
}

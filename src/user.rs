use crate::*;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub pool_mode: Option<String>,
    pub max_user_connections: Option<i32>,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} =", self.name)?;
        self.pool_mode
            .as_ref()
            .map(|pool_mode| write!(f, " pool_mode={pool_mode}"))
            .transpose()?;
        self.max_user_connections
            .map(|max_user_connections| write!(f, " max_user_connections={max_user_connections}"))
            .transpose()?;
        Ok(())
    }
}

impl User {
    pub fn all() -> Result<BTreeMap<i32, Vec<User>>> {
        Spi::connect(|client| {
            let tup_table = client.select(
                "SELECT * FROM pgbouncer.users ORDER BY group_id, name",
                None,
                None,
            )?;

            let mut users = BTreeMap::new();
            let mut prev_group_id_opt = None;
            let mut users_for_last_group: Vec<User> = Vec::new();
            for tuple in tup_table {
                let group_id = tuple
                    .get_datum_by_name("group_id")?
                    .value::<i32>()?
                    .context("no group_id")?;

                if prev_group_id_opt != Some(group_id) {
                    if let Some(prev_group_id) = prev_group_id_opt {
                        users.insert(prev_group_id, users_for_last_group);
                        users_for_last_group = Vec::new();
                    }
                    prev_group_id_opt = Some(group_id);
                }

                let user = User {
                    name: tuple
                        .get_datum_by_name("name")?
                        .value::<String>()?
                        .context("no name")?,
                    pool_mode: tuple.get_datum_by_name("pool_mode")?.value::<String>()?,
                    max_user_connections: tuple
                        .get_datum_by_name("max_user_connections")?
                        .value::<i32>()?,
                };

                users_for_last_group.push(user);
            }

            if let Some(prev_group_id) = prev_group_id_opt {
                if !users_for_last_group.is_empty() {
                    users.insert(prev_group_id, users_for_last_group);
                }
            }
            Ok(users)
        })
    }
}

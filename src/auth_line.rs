use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthLine {
    pub user_name: String,
    pub password: String,
}

impl Display for AuthLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#""{}" "{}""#,
            escape_double_quotes(&self.user_name),
            escape_double_quotes(&self.password)
        )
    }
}

impl AuthLine {
    pub fn all() -> Result<BTreeMap<i32, Vec<AuthLine>>> {
        Spi::connect(|client| {
            let tup_table = client.select(
                "SELECT * from pgbouncer.auth ORDER BY group_id, user_name",
                None,
                None,
            )?;

            let mut auth_lines = BTreeMap::new();
            let mut prev_group_id_opt = None;
            let mut auth_lines_for_last_group: Vec<AuthLine> = Vec::new();
            for tuple in tup_table {
                let group_id = tuple
                    .get_datum_by_name("group_id")?
                    .value::<i32>()?
                    .context("no group_id")?;

                if prev_group_id_opt != Some(group_id) {
                    if let Some(prev_group_id) = prev_group_id_opt {
                        auth_lines.insert(prev_group_id, auth_lines_for_last_group);
                        auth_lines_for_last_group = Vec::new();
                    }
                    prev_group_id_opt = Some(group_id);
                }

                let auth_line = AuthLine {
                    user_name: tuple
                        .get_datum_by_name("user_name")?
                        .value::<String>()?
                        .context("no user")?,
                    password: tuple
                        .get_datum_by_name("password")?
                        .value::<String>()?
                        .context("no password")?,
                };

                auth_lines_for_last_group.push(auth_line);
            }
            if let Some(prev_group_id) = prev_group_id_opt {
                if !auth_lines_for_last_group.is_empty() {
                    auth_lines.insert(prev_group_id, auth_lines_for_last_group);
                }
            };
            Ok(auth_lines)
        })
    }
}

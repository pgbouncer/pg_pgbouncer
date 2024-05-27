use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HbaRule {
    pub r#type: String,
    pub database: Vec<String>,
    pub user_name: Vec<String>,
    pub address: Option<String>,
    pub method: String,
}

impl Display for HbaRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            self.r#type,
            self.database.join(","),
            self.user_name.join(","),
            self.address.as_deref().unwrap_or(""),
            self.method
        )
    }
}

impl HbaRule {
    pub fn all() -> Result<BTreeMap<i32, Vec<HbaRule>>> {
        Spi::connect(|client| {
            let tup_table = client.select(
                "SELECT * from pgbouncer.hba ORDER BY group_id, position",
                None,
                None,
            )?;

            let mut hba_rules = BTreeMap::new();
            let mut prev_group_id_opt = None;
            let mut hba_rules_for_last_group: Vec<HbaRule> = Vec::new();
            for tuple in tup_table {
                let group_id = tuple
                    .get_datum_by_name("group_id")?
                    .value::<i32>()?
                    .context("no group_id")?;

                if prev_group_id_opt != Some(group_id) {
                    if let Some(prev_group_id) = prev_group_id_opt {
                        hba_rules.insert(prev_group_id, hba_rules_for_last_group);
                        hba_rules_for_last_group = Vec::new();
                    }
                    prev_group_id_opt = Some(group_id);
                }

                let hba_rule = HbaRule {
                    r#type: tuple
                        .get_datum_by_name("type")?
                        .value::<String>()?
                        .context("no type")?,
                    database: tuple
                        .get_datum_by_name("database")?
                        .value::<Vec<String>>()?
                        .context("no database")?,
                    user_name: tuple
                        .get_datum_by_name("user_name")?
                        .value::<Vec<String>>()?
                        .context("no user_name")?,
                    address: tuple.get_datum_by_name("address")?.value::<String>()?,
                    method: tuple
                        .get_datum_by_name("method")?
                        .value::<String>()?
                        .context("no method")?,
                };
                hba_rules_for_last_group.push(hba_rule);
            }

            if let Some(prev_group_id) = prev_group_id_opt {
                if !hba_rules_for_last_group.is_empty() {
                    hba_rules.insert(prev_group_id, hba_rules_for_last_group);
                }
            };

            Ok(hba_rules)
        })
    }
}

use crate::*;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub password: Option<String>,
    pub auth_user: Option<String>,
    pub pool_size: Option<i32>,
    pub min_pool_size: Option<i32>,
    pub reserve_pool: Option<i32>,
    pub connect_query: Option<String>,
    pub pool_mode: Option<String>,
    pub max_db_connections: Option<i32>,
    pub client_encoding: Option<String>,
    pub datestyle: Option<String>,
    pub timezone: Option<String>,
}

impl Display for Database {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.name == "*" {
            write!(f, "* =")?;
        } else if contains_only_ascii_alphanumeric_and_underscore(&self.name) {
            write!(f, "{} =", self.name)?;
        } else {
            // TODO: Support for other datase names requires fix in pgbouncer ini parser:
            // https://github.com/pgbouncer/pgbouncer/issues/971
            panic!("invalid database name: {}", self.name);
        }
        self.host
            .as_ref()
            .map(|host| write!(f, " host={host}"))
            .transpose()?;
        self.port
            .map(|port| write!(f, " port={port}"))
            .transpose()?;
        self.password
            .as_ref()
            .map(|password| write!(f, " password={password}"))
            .transpose()?;
        self.auth_user
            .as_ref()
            .map(|auth_user| write!(f, " auth_user={auth_user}"))
            .transpose()?;
        self.pool_size
            .map(|pool_size| write!(f, " pool_size={pool_size}"))
            .transpose()?;
        self.min_pool_size
            .map(|min_pool_size| write!(f, " min_pool_size={min_pool_size}"))
            .transpose()?;
        self.reserve_pool
            .map(|reserve_pool| write!(f, " reserve_pool={reserve_pool}"))
            .transpose()?;
        self.connect_query
            .as_ref()
            .map(|connect_query| write!(f, " connect_query={connect_query}"))
            .transpose()?;
        self.pool_mode
            .as_ref()
            .map(|pool_mode| write!(f, " pool_mode={pool_mode}"))
            .transpose()?;
        self.max_db_connections
            .map(|max_db_connections| write!(f, " max_db_connections={max_db_connections}"))
            .transpose()?;
        self.client_encoding
            .as_ref()
            .map(|client_encoding| write!(f, " client_encoding={client_encoding}"))
            .transpose()?;
        self.datestyle
            .as_ref()
            .map(|datestyle| write!(f, " datestyle={datestyle}"))
            .transpose()?;
        self.timezone
            .as_ref()
            .map(|timezone| write!(f, " timezone={timezone}"))
            .transpose()?;
        Ok(())
    }
}

impl Database {
    pub fn all() -> Result<BTreeMap<i32, Vec<Database>>> {
        Spi::connect(|client| {
            let tuple_table = client.select(
                "SELECT * FROM pgbouncer.databases ORDER BY group_id, name",
                None,
                None,
            )?;

            let mut databases = BTreeMap::new();
            let mut prev_group_id_opt = None;
            let mut database_for_last_group: Vec<Database> = Vec::new();

            for tuple in tuple_table {
                let group_id = tuple
                    .get_datum_by_name("group_id")?
                    .value::<i32>()?
                    .context("no group_id")?;
                if Some(group_id) != prev_group_id_opt {
                    if let Some(prev_group_id) = prev_group_id_opt {
                        databases.insert(prev_group_id, database_for_last_group);
                        database_for_last_group = Vec::new();
                    }
                    prev_group_id_opt = Some(group_id);
                }

                let db = Database {
                    name: tuple
                        .get_datum_by_name("name")?
                        .value::<String>()?
                        .context("no name")?,
                    host: tuple.get_datum_by_name("host")?.value::<String>()?,
                    port: tuple.get_datum_by_name("port")?.value::<i32>()?,
                    password: tuple.get_datum_by_name("password")?.value::<String>()?,
                    auth_user: tuple.get_datum_by_name("auth_user")?.value::<String>()?,
                    pool_size: tuple.get_datum_by_name("pool_size")?.value::<i32>()?,
                    min_pool_size: tuple.get_datum_by_name("min_pool_size")?.value::<i32>()?,
                    reserve_pool: tuple.get_datum_by_name("reserve_pool")?.value::<i32>()?,
                    connect_query: tuple
                        .get_datum_by_name("connect_query")?
                        .value::<String>()?,
                    pool_mode: tuple.get_datum_by_name("pool_mode")?.value::<String>()?,
                    max_db_connections: tuple
                        .get_datum_by_name("max_db_connections")?
                        .value::<i32>()?,
                    client_encoding: tuple
                        .get_datum_by_name("client_encoding")?
                        .value::<String>()?,
                    datestyle: tuple.get_datum_by_name("datestyle")?.value::<String>()?,
                    timezone: tuple.get_datum_by_name("timezone")?.value::<String>()?,
                };

                database_for_last_group.push(db);
            }

            if let Some(prev_group_id) = prev_group_id_opt {
                if !database_for_last_group.is_empty() {
                    databases.insert(prev_group_id, database_for_last_group);
                }
            }

            Ok(databases)
        })
    }
}

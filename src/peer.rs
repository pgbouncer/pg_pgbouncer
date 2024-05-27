use crate::*;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: i32,
    pub host: String,
    pub port: i32,
    pub pool_size: Option<i32>,
}

impl Display for Peer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = host={} port={}",
            self.peer_id, self.host, self.port
        )?;
        self.pool_size
            .as_ref()
            .map(|pool_size| write!(f, " pool_size={pool_size}"))
            .transpose()?;
        Ok(())
    }
}

impl Peer {
    pub fn all() -> Result<BTreeMap<i32, Vec<Peer>>> {
        Spi::connect(|client| {
            let tup_table = client.select("SELECT * FROM pgbouncer.peers", None, None)?;
            let mut peers = BTreeMap::new();
            let mut prev_group_id_opt = None;
            let mut peers_for_last_group: Vec<Peer> = Vec::new();
            for tuple in tup_table {
                let group_id = tuple
                    .get_datum_by_name("group_id")?
                    .value::<i32>()?
                    .context("no group_id")?;

                if prev_group_id_opt != Some(group_id) {
                    if let Some(prev_group_id) = prev_group_id_opt {
                        peers.insert(prev_group_id, peers_for_last_group);
                        peers_for_last_group = Vec::new();
                    }
                    prev_group_id_opt = Some(group_id);
                }

                let peer = Peer {
                    peer_id: tuple
                        .get_datum_by_name("peer_id")?
                        .value::<i32>()?
                        .context("no peer_id")?,
                    host: tuple
                        .get_datum_by_name("host")?
                        .value::<String>()?
                        .context("no host")?,
                    port: tuple
                        .get_datum_by_name("port")?
                        .value::<i32>()?
                        .context("no port")?,
                    pool_size: tuple.get_datum_by_name("pool_size")?.value::<i32>()?,
                };

                peers_for_last_group.push(peer);
            }

            if let Some(prev_group_id) = prev_group_id_opt {
                if !peers_for_last_group.is_empty() {
                    peers.insert(prev_group_id, peers_for_last_group);
                }
            };

            Ok(peers)
        })
    }
}

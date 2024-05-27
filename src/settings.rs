use crate::*;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct Settings {
    admin_users: Option<Vec<String>>,
    application_name_add_host: Option<bool>,
    auth_dbname: Option<String>,
    auth_query: Option<String>,
    pub auth_type: Option<String>,
    auth_user: Option<String>,
    autodb_idle_timeout: Option<Interval>,
    client_idle_timeout: Option<Interval>,
    client_login_timeout: Option<Interval>,
    client_tls_ca_file: Option<String>,
    client_tls_cert_file: Option<String>,
    client_tls_ciphers: Option<String>,
    client_tls_dheparams: Option<String>,
    client_tls_ecdhcurve: Option<String>,
    client_tls_key_file: Option<String>,
    client_tls_protocols: Option<String>,
    client_tls_sslmode: Option<String>,
    default_pool_size: Option<i32>,
    disable_pqexec: Option<bool>,
    dns_max_ttl: Option<Interval>,
    dns_nxdomain_ttl: Option<Interval>,
    dns_zone_check_period: Option<Interval>,
    idle_transaction_timeout: Option<Interval>,
    ignore_startup_parameters: Option<Vec<String>>,
    listen_addr: Option<Vec<String>>,
    listen_backlog: Option<i32>,
    listen_port: Option<i32>,
    log_connections: Option<bool>,
    log_disconnections: Option<bool>,
    log_pooler_errors: Option<bool>,
    log_stats: Option<bool>,
    logfile: Option<String>,
    max_client_conn: Option<i32>,
    max_db_connections: Option<i32>,
    max_packet_size: Option<i32>,
    max_prepared_statements: Option<i32>,
    max_user_connections: Option<i32>,
    min_pool_size: Option<i32>,
    pkt_buf: Option<i32>,
    pool_mode: Option<String>,
    query_timeout: Option<Interval>,
    query_wait_timeout: Option<Interval>,
    cancel_wait_timeout: Option<Interval>,
    reserve_pool_size: Option<i32>,
    reserve_pool_timeout: Option<Interval>,
    resolv_conf: Option<String>,
    sbuf_loopcnt: Option<i32>,
    server_check_delay: Option<Interval>,
    server_check_query: Option<String>,
    server_connect_timeout: Option<Interval>,
    server_fast_close: Option<bool>,
    server_idle_timeout: Option<Interval>,
    server_lifetime: Option<Interval>,
    server_login_retry: Option<String>,
    server_reset_query: Option<String>,
    server_reset_query_always: Option<bool>,
    server_round_robin: Option<bool>,
    server_tls_ca_file: Option<String>,
    server_tls_cert_file: Option<String>,
    server_tls_ciphers: Option<String>,
    server_tls_key_file: Option<String>,
    server_tls_protocols: Option<String>,
    server_tls_sslmode: Option<String>,
    stats_period: Option<i32>,
    stats_users: Option<Vec<String>>,
    suspend_timeout: Option<Interval>,
    syslog: Option<bool>,
    syslog_facility: Option<String>,
    syslog_ident: Option<String>,
    tcp_defer_accept: Option<bool>,
    tcp_keepalive: Option<bool>,
    tcp_keepcnt: Option<i32>,
    tcp_keepidle: Option<i32>,
    tcp_keepintvl: Option<i32>,
    tcp_socket_buffer: Option<i32>,
    tcp_user_timeout: Option<i32>,
    track_extra_parameters: Option<Vec<String>>,
    unix_socket_group: Option<String>,
    unix_socket_mode: Option<i32>,
    verbose: Option<i32>,
}

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.admin_users
            .as_ref()
            .map(|val| writeln!(f, "admin_users = {}", val.join(", ")))
            .transpose()?;
        self.application_name_add_host
            .map(|val| writeln!(f, "application_name_add_host = {}", val as i32))
            .transpose()?;
        self.auth_dbname
            .as_ref()
            .map(|val| writeln!(f, "auth_dbname = {val}"))
            .transpose()?;
        self.auth_query
            .as_ref()
            .map(|val| writeln!(f, "auth_query = {val}"))
            .transpose()?;
        self.auth_type
            .as_ref()
            .map(|val| writeln!(f, "auth_type = {val}"))
            .transpose()?;
        self.auth_user
            .as_ref()
            .map(|val| writeln!(f, "auth_user = {val}"))
            .transpose()?;
        self.autodb_idle_timeout
            .map(|val| writeln!(f, "autodb_idle_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.client_idle_timeout
            .map(|val| writeln!(f, "client_idle_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.client_login_timeout
            .map(|val| writeln!(f, "client_login_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.client_tls_ca_file
            .as_ref()
            .map(|val| writeln!(f, "client_tls_ca_file = {val}"))
            .transpose()?;
        self.client_tls_cert_file
            .as_ref()
            .map(|val| writeln!(f, "client_tls_cert_file = {val}"))
            .transpose()?;
        self.client_tls_ciphers
            .as_ref()
            .map(|val| writeln!(f, "client_tls_ciphers = {val}"))
            .transpose()?;
        self.client_tls_dheparams
            .as_ref()
            .map(|val| writeln!(f, "client_tls_dheparams = {val}"))
            .transpose()?;
        self.client_tls_ecdhcurve
            .as_ref()
            .map(|val| writeln!(f, "client_tls_ecdhcurve = {val}"))
            .transpose()?;
        self.client_tls_key_file
            .as_ref()
            .map(|val| writeln!(f, "client_tls_key_file = {val}"))
            .transpose()?;
        self.client_tls_protocols
            .as_ref()
            .map(|val| writeln!(f, "client_tls_protocols = {val}"))
            .transpose()?;
        self.client_tls_sslmode
            .as_ref()
            .map(|val| writeln!(f, "client_tls_sslmode = {val}"))
            .transpose()?;
        self.default_pool_size
            .map(|val| writeln!(f, "default_pool_size = {val}"))
            .transpose()?;
        self.disable_pqexec
            .map(|val| writeln!(f, "disable_pqexec = {}", val as i32))
            .transpose()?;
        self.dns_max_ttl
            .map(|val| writeln!(f, "dns_max_ttl = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.dns_nxdomain_ttl
            .map(|val| writeln!(f, "dns_nxdomain_ttl = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.dns_zone_check_period
            .map(|val| writeln!(f, "dns_zone_check_period = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.idle_transaction_timeout
            .map(|val| {
                writeln!(
                    f,
                    "idle_transaction_timeout = {}",
                    val.as_micros() / 1_000_000
                )
            })
            .transpose()?;
        self.ignore_startup_parameters
            .as_ref()
            .map(|val| writeln!(f, "ignore_startup_parameters = {}", val.join(", ")))
            .transpose()?;
        self.listen_addr
            .as_ref()
            .map(|val| writeln!(f, "listen_addr = {}", val.join(", ")))
            .transpose()?;
        self.listen_backlog
            .map(|val| writeln!(f, "listen_backlog = {val}"))
            .transpose()?;
        self.listen_port
            .map(|val| writeln!(f, "listen_port = {val}"))
            .transpose()?;
        self.log_connections
            .map(|val| writeln!(f, "log_connections = {}", val as i32))
            .transpose()?;
        self.log_disconnections
            .map(|val| writeln!(f, "log_disconnections = {}", val as i32))
            .transpose()?;
        self.log_pooler_errors
            .map(|val| writeln!(f, "log_pooler_errors = {}", val as i32))
            .transpose()?;
        self.log_stats
            .map(|val| writeln!(f, "log_stats = {}", val as i32))
            .transpose()?;
        self.logfile
            .as_ref()
            .map(|val| writeln!(f, "logfile = {val}"))
            .transpose()?;
        self.max_client_conn
            .map(|val| writeln!(f, "max_client_conn = {val}"))
            .transpose()?;
        self.max_db_connections
            .map(|val| writeln!(f, "max_db_connections = {val}"))
            .transpose()?;
        self.max_packet_size
            .map(|val| writeln!(f, "max_packet_size = {val}"))
            .transpose()?;
        self.max_user_connections
            .map(|val| writeln!(f, "max_user_connections = {val}"))
            .transpose()?;
        self.min_pool_size
            .map(|val| writeln!(f, "min_pool_size = {val}"))
            .transpose()?;
        self.pkt_buf
            .map(|val| writeln!(f, "pkt_buf = {val}"))
            .transpose()?;
        self.pool_mode
            .as_ref()
            .map(|val| writeln!(f, "pool_mode = {val}"))
            .transpose()?;
        self.max_prepared_statements
            .map(|val| writeln!(f, "max_prepared_statements = {val}"))
            .transpose()?;
        self.query_timeout
            .map(|val| writeln!(f, "query_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.query_wait_timeout
            .map(|val| writeln!(f, "query_wait_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.cancel_wait_timeout
            .map(|val| writeln!(f, "cancel_wait_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.reserve_pool_size
            .map(|val| writeln!(f, "reserve_pool_size = {val}"))
            .transpose()?;
        self.reserve_pool_timeout
            .map(|val| writeln!(f, "reserve_pool_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.resolv_conf
            .as_ref()
            .map(|val| writeln!(f, "resolv_conf = {val}"))
            .transpose()?;
        self.sbuf_loopcnt
            .map(|val| writeln!(f, "sbuf_loopcnt = {val}"))
            .transpose()?;
        self.server_check_delay
            .map(|val| writeln!(f, "server_check_delay = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.server_check_query
            .as_ref()
            .map(|val| writeln!(f, "server_check_query = {val}"))
            .transpose()?;
        self.server_connect_timeout
            .map(|val| {
                writeln!(
                    f,
                    "server_connect_timeout = {}",
                    val.as_micros() / 1_000_000
                )
            })
            .transpose()?;
        self.server_fast_close
            .map(|val| writeln!(f, "server_fast_close = {}", val as i32))
            .transpose()?;
        self.server_idle_timeout
            .map(|val| writeln!(f, "server_idle_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.server_lifetime
            .map(|val| writeln!(f, "server_lifetime = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.server_login_retry
            .as_ref()
            .map(|val| writeln!(f, "server_login_retry = {val}"))
            .transpose()?;
        self.server_reset_query
            .as_ref()
            .map(|val| writeln!(f, "server_reset_query = {val}"))
            .transpose()?;
        self.server_reset_query_always
            .map(|val| writeln!(f, "server_reset_query_always = {}", val as i32))
            .transpose()?;
        self.server_round_robin
            .map(|val| writeln!(f, "server_round_robin = {}", val as i32))
            .transpose()?;
        self.server_tls_ca_file
            .as_ref()
            .map(|val| writeln!(f, "server_tls_ca_file = {val}"))
            .transpose()?;
        self.server_tls_cert_file
            .as_ref()
            .map(|val| writeln!(f, "server_tls_cert_file = {val}"))
            .transpose()?;
        self.server_tls_ciphers
            .as_ref()
            .map(|val| writeln!(f, "server_tls_ciphers = {val}"))
            .transpose()?;
        self.server_tls_key_file
            .as_ref()
            .map(|val| writeln!(f, "server_tls_key_file = {val}"))
            .transpose()?;
        self.server_tls_protocols
            .as_ref()
            .map(|val| writeln!(f, "server_tls_protocols = {val}"))
            .transpose()?;
        self.server_tls_sslmode
            .as_ref()
            .map(|val| writeln!(f, "server_tls_sslmode = {val}"))
            .transpose()?;
        self.stats_period
            .map(|val| writeln!(f, "stats_period = {val}"))
            .transpose()?;
        self.stats_users
            .as_ref()
            .map(|val| writeln!(f, "stats_users = {}", val.join(", ")))
            .transpose()?;
        self.suspend_timeout
            .map(|val| writeln!(f, "suspend_timeout = {}", val.as_micros() / 1_000_000))
            .transpose()?;
        self.syslog
            .map(|val| writeln!(f, "syslog = {}", val as i32))
            .transpose()?;
        self.syslog_facility
            .as_ref()
            .map(|val| writeln!(f, "syslog_facility = {val}"))
            .transpose()?;
        self.syslog_ident
            .as_ref()
            .map(|val| writeln!(f, "syslog_ident = {val}"))
            .transpose()?;
        self.tcp_defer_accept
            .map(|val| writeln!(f, "tcp_defer_accept = {}", val as i32))
            .transpose()?;
        self.tcp_keepalive
            .map(|val| writeln!(f, "tcp_keepalive = {}", val as i32))
            .transpose()?;
        self.tcp_keepcnt
            .map(|val| writeln!(f, "tcp_keepcnt = {val}"))
            .transpose()?;
        self.tcp_keepidle
            .map(|val| writeln!(f, "tcp_keepidle = {val}"))
            .transpose()?;
        self.tcp_keepintvl
            .map(|val| writeln!(f, "tcp_keepintvl = {val}"))
            .transpose()?;
        self.tcp_socket_buffer
            .map(|val| writeln!(f, "tcp_socket_buffer = {val}"))
            .transpose()?;
        self.tcp_user_timeout
            .map(|val| writeln!(f, "tcp_user_timeout = {val}"))
            .transpose()?;
        self.track_extra_parameters
            .as_ref()
            .map(|val| writeln!(f, "track_extra_parameters = {}", val.join(", ")))
            .transpose()?;
        self.unix_socket_group
            .as_ref()
            .map(|val| writeln!(f, "unix_socket_group = {val}"))
            .transpose()?;
        self.unix_socket_mode
            .map(|val| writeln!(f, "unix_socket_mode = {val}"))
            .transpose()?;
        self.verbose
            .map(|val| writeln!(f, "verbose = {val}"))
            .transpose()?;
        Ok(())
    }
}

impl Settings {
    pub fn all() -> Result<BTreeMap<i32, Settings>> {
        Spi::connect(|client| {
            let tup_table = client.select(
                indoc!(
                    r#"SELECT * FROM pgbouncer.settings ORDER BY group_id;
                "#
                ),
                None,
                None,
            )?;

            let mut group_settings = BTreeMap::new();
            for tuple in tup_table {
                let group_id = tuple
                    .get_datum_by_name("group_id")?
                    .value::<i32>()?
                    .context("no group_id")?;
                let settings = Settings {
                    admin_users: tuple
                        .get_datum_by_name("admin_users")?
                        .value::<Vec<String>>()?,
                    application_name_add_host: tuple
                        .get_datum_by_name("application_name_add_host")?
                        .value::<bool>()?,
                    auth_dbname: tuple.get_datum_by_name("auth_dbname")?.value::<String>()?,
                    auth_query: tuple.get_datum_by_name("auth_query")?.value::<String>()?,
                    auth_type: tuple.get_datum_by_name("auth_type")?.value::<String>()?,
                    auth_user: tuple.get_datum_by_name("auth_user")?.value::<String>()?,
                    autodb_idle_timeout: tuple
                        .get_datum_by_name("autodb_idle_timeout")?
                        .value::<Interval>()?,
                    client_idle_timeout: tuple
                        .get_datum_by_name("client_idle_timeout")?
                        .value::<Interval>()?,
                    client_login_timeout: tuple
                        .get_datum_by_name("client_login_timeout")?
                        .value::<Interval>()?,
                    client_tls_ca_file: tuple
                        .get_datum_by_name("client_tls_ca_file")?
                        .value::<String>()?,
                    client_tls_cert_file: tuple
                        .get_datum_by_name("client_tls_cert_file")?
                        .value::<String>()?,
                    client_tls_ciphers: tuple
                        .get_datum_by_name("client_tls_ciphers")?
                        .value::<String>()?,
                    client_tls_dheparams: tuple
                        .get_datum_by_name("client_tls_dheparams")?
                        .value::<String>()?,
                    client_tls_ecdhcurve: tuple
                        .get_datum_by_name("client_tls_ecdhcurve")?
                        .value::<String>()?,
                    client_tls_key_file: tuple
                        .get_datum_by_name("client_tls_key_file")?
                        .value::<String>()?,
                    client_tls_protocols: tuple
                        .get_datum_by_name("client_tls_protocols")?
                        .value::<String>()?,
                    client_tls_sslmode: tuple
                        .get_datum_by_name("client_tls_sslmode")?
                        .value::<String>()?,
                    default_pool_size: tuple
                        .get_datum_by_name("default_pool_size")?
                        .value::<i32>()?,
                    disable_pqexec: tuple.get_datum_by_name("disable_pqexec")?.value::<bool>()?,
                    dns_max_ttl: tuple
                        .get_datum_by_name("dns_max_ttl")?
                        .value::<Interval>()?,
                    dns_nxdomain_ttl: tuple
                        .get_datum_by_name("dns_nxdomain_ttl")?
                        .value::<Interval>()?,
                    dns_zone_check_period: tuple
                        .get_datum_by_name("dns_zone_check_period")?
                        .value::<Interval>()?,
                    idle_transaction_timeout: tuple
                        .get_datum_by_name("idle_transaction_timeout")?
                        .value::<Interval>()?,
                    ignore_startup_parameters: tuple
                        .get_datum_by_name("ignore_startup_parameters")?
                        .value::<Vec<String>>()?,
                    listen_addr: tuple
                        .get_datum_by_name("listen_addr")?
                        .value::<Vec<String>>()?,
                    listen_backlog: tuple.get_datum_by_name("listen_backlog")?.value::<i32>()?,
                    listen_port: tuple.get_datum_by_name("listen_port")?.value::<i32>()?,
                    log_connections: tuple
                        .get_datum_by_name("log_connections")?
                        .value::<bool>()?,
                    log_disconnections: tuple
                        .get_datum_by_name("log_disconnections")?
                        .value::<bool>()?,
                    log_pooler_errors: tuple
                        .get_datum_by_name("log_pooler_errors")?
                        .value::<bool>()?,
                    log_stats: tuple.get_datum_by_name("log_stats")?.value::<bool>()?,
                    logfile: tuple.get_datum_by_name("logfile")?.value::<String>()?,
                    max_client_conn: tuple.get_datum_by_name("max_client_conn")?.value::<i32>()?,
                    max_db_connections: tuple
                        .get_datum_by_name("max_db_connections")?
                        .value::<i32>()?,
                    max_packet_size: tuple.get_datum_by_name("max_packet_size")?.value::<i32>()?,
                    max_user_connections: tuple
                        .get_datum_by_name("max_user_connections")?
                        .value::<i32>()?,
                    min_pool_size: tuple.get_datum_by_name("min_pool_size")?.value::<i32>()?,
                    pkt_buf: tuple.get_datum_by_name("pkt_buf")?.value::<i32>()?,
                    pool_mode: tuple.get_datum_by_name("pool_mode")?.value::<String>()?,
                    max_prepared_statements: tuple
                        .get_datum_by_name("max_prepared_statements")?
                        .value::<i32>()?,
                    query_timeout: tuple
                        .get_datum_by_name("query_timeout")?
                        .value::<Interval>()?,
                    query_wait_timeout: tuple
                        .get_datum_by_name("query_wait_timeout")?
                        .value::<Interval>()?,
                    cancel_wait_timeout: tuple
                        .get_datum_by_name("cancel_wait_timeout")?
                        .value::<Interval>()?,
                    reserve_pool_size: tuple
                        .get_datum_by_name("reserve_pool_size")?
                        .value::<i32>()?,
                    reserve_pool_timeout: tuple
                        .get_datum_by_name("reserve_pool_timeout")?
                        .value::<Interval>()?,
                    resolv_conf: tuple.get_datum_by_name("resolv_conf")?.value::<String>()?,
                    sbuf_loopcnt: tuple.get_datum_by_name("sbuf_loopcnt")?.value::<i32>()?,
                    server_check_delay: tuple
                        .get_datum_by_name("server_check_delay")?
                        .value::<Interval>()?,
                    server_check_query: tuple
                        .get_datum_by_name("server_check_query")?
                        .value::<String>()?,
                    server_connect_timeout: tuple
                        .get_datum_by_name("server_connect_timeout")?
                        .value::<Interval>()?,
                    server_fast_close: tuple
                        .get_datum_by_name("server_fast_close")?
                        .value::<bool>()?,
                    server_idle_timeout: tuple
                        .get_datum_by_name("server_idle_timeout")?
                        .value::<Interval>()?,
                    server_lifetime: tuple
                        .get_datum_by_name("server_lifetime")?
                        .value::<Interval>()?,
                    server_login_retry: tuple
                        .get_datum_by_name("server_login_retry")?
                        .value::<String>()?,
                    server_reset_query: tuple
                        .get_datum_by_name("server_reset_query")?
                        .value::<String>()?,
                    server_reset_query_always: tuple
                        .get_datum_by_name("server_reset_query_always")?
                        .value::<bool>()?,
                    server_round_robin: tuple
                        .get_datum_by_name("server_round_robin")?
                        .value::<bool>()?,
                    server_tls_ca_file: tuple
                        .get_datum_by_name("server_tls_ca_file")?
                        .value::<String>()?,
                    server_tls_cert_file: tuple
                        .get_datum_by_name("server_tls_cert_file")?
                        .value::<String>()?,
                    server_tls_ciphers: tuple
                        .get_datum_by_name("server_tls_ciphers")?
                        .value::<String>()?,
                    server_tls_key_file: tuple
                        .get_datum_by_name("server_tls_key_file")?
                        .value::<String>()?,
                    server_tls_protocols: tuple
                        .get_datum_by_name("server_tls_protocols")?
                        .value::<String>()?,
                    server_tls_sslmode: tuple
                        .get_datum_by_name("server_tls_sslmode")?
                        .value::<String>()?,
                    stats_period: tuple.get_datum_by_name("stats_period")?.value::<i32>()?,
                    stats_users: tuple
                        .get_datum_by_name("stats_users")?
                        .value::<Vec<String>>()?,
                    suspend_timeout: tuple
                        .get_datum_by_name("suspend_timeout")?
                        .value::<Interval>()?,
                    syslog: tuple.get_datum_by_name("syslog")?.value::<bool>()?,
                    syslog_facility: tuple
                        .get_datum_by_name("syslog_facility")?
                        .value::<String>()?,
                    syslog_ident: tuple.get_datum_by_name("syslog_ident")?.value::<String>()?,
                    tcp_defer_accept: tuple
                        .get_datum_by_name("tcp_defer_accept")?
                        .value::<bool>()?,
                    tcp_keepalive: tuple.get_datum_by_name("tcp_keepalive")?.value::<bool>()?,
                    tcp_keepcnt: tuple.get_datum_by_name("tcp_keepcnt")?.value::<i32>()?,
                    tcp_keepidle: tuple.get_datum_by_name("tcp_keepidle")?.value::<i32>()?,
                    tcp_keepintvl: tuple.get_datum_by_name("tcp_keepintvl")?.value::<i32>()?,
                    tcp_socket_buffer: tuple
                        .get_datum_by_name("tcp_socket_buffer")?
                        .value::<i32>()?,
                    tcp_user_timeout: tuple
                        .get_datum_by_name("tcp_user_timeout")?
                        .value::<i32>()?,
                    track_extra_parameters: tuple
                        .get_datum_by_name("track_extra_parameters")?
                        .value::<Vec<String>>()?,
                    unix_socket_group: tuple
                        .get_datum_by_name("unix_socket_group")?
                        .value::<String>()?,
                    unix_socket_mode: tuple
                        .get_datum_by_name("unix_socket_mode")?
                        .value::<i32>()?,
                    verbose: tuple.get_datum_by_name("verbose")?.value::<i32>()?,
                };
                group_settings.insert(group_id, settings);
            }

            Ok(group_settings)
        })
    }
}

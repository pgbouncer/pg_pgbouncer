CREATE TABLE pgbouncer.groups (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name text UNIQUE NOT NULL,
    enabled boolean NOT NULL DEFAULT true,
    num_processes int,
    peer_offset int NOT NULL DEFAULT 0,
    stay_alive boolean NOT NULL DEFAULT true,
    restart_target bigint NOT NULL DEFAULT 0
);

CREATE TABLE pgbouncer.databases (
    group_id int NOT NULL REFERENCES pgbouncer.groups (id) ON DELETE CASCADE,
    name text NOT NULL,
    host text,
    port int,
    password text,
    auth_user text,
    pool_size int,
    min_pool_size int,
    reserve_pool int,
    connect_query text,
    pool_mode text,
    max_db_connections int,
    client_encoding text,
    datestyle text,
    timezone text,
    PRIMARY KEY (group_id, name)
);

CREATE TABLE pgbouncer.users (
    group_id int NOT NULL REFERENCES pgbouncer.groups (id) ON DELETE CASCADE,
    name text NOT NULL,
    pool_mode text,
    max_user_connections int,
    PRIMARY KEY (group_id, name)
);

CREATE TABLE pgbouncer.peers (
    group_id int NOT NULL REFERENCES pgbouncer.groups (id) ON DELETE CASCADE,
    peer_id int NOT NULL,
    host text NOT NULL,
    port int NOT NULL DEFAULT 6432,
    pool_size int,
    PRIMARY KEY (group_id, peer_id)
);

CREATE TABLE pgbouncer.auth (
    group_id int NOT NULL REFERENCES pgbouncer.groups (id) ON DELETE CASCADE,
    user_name text NOT NULL,
    password text NOT NULL,
    PRIMARY KEY (group_id, user_name)
);

CREATE TABLE pgbouncer.hba (
    group_id int NOT NULL REFERENCES pgbouncer.groups (id) ON DELETE CASCADE,
    position numeric,
    type text NOT NULL,
    database text[] NOT NULL,
    user_name text[] NOT NULL,
    address text,
    method text NOT NULL,
    PRIMARY KEY (group_id, position)
);

CREATE TABLE pgbouncer.settings (
    group_id int
        NOT NULL
        REFERENCES pgbouncer.groups (id)
            ON DELETE CASCADE
            DEFERRABLE INITIALLY DEFERRED
        PRIMARY KEY,
    admin_users text[],
    application_name_add_host boolean,
    auth_dbname text,
    auth_query text,
    auth_type text,
    auth_user text,
    autodb_idle_timeout interval,
    client_idle_timeout interval,
    client_login_timeout interval,
    client_tls_ca_file text,
    client_tls_cert_file text,
    client_tls_ciphers text,
    client_tls_dheparams text,
    client_tls_ecdhcurve text,
    client_tls_key_file text,
    client_tls_protocols text,
    client_tls_sslmode text,
    default_pool_size int,
    disable_pqexec bool,
    dns_max_ttl interval,
    dns_nxdomain_ttl interval,
    dns_zone_check_period interval,
    idle_transaction_timeout interval,
    ignore_startup_parameters text[],
    listen_addr text[],
    listen_backlog int,
    listen_port int,
    log_connections boolean,
    log_disconnections boolean,
    log_pooler_errors boolean,
    log_stats boolean,
    logfile text,
    max_client_conn int,
    max_db_connections int,
    max_packet_size int,
    max_prepared_statements int,
    max_user_connections int,
    min_pool_size int,
    pkt_buf int,
    pool_mode text,
    query_timeout interval,
    query_wait_timeout interval,
    cancel_wait_timeout interval,
    reserve_pool_size int,
    reserve_pool_timeout interval,
    resolv_conf text,
    sbuf_loopcnt int,
    server_check_delay interval,
    server_check_query text,
    server_connect_timeout interval,
    server_fast_close boolean,
    server_idle_timeout interval,
    server_lifetime interval,
    server_login_retry text,
    server_reset_query text,
    server_reset_query_always boolean,
    server_round_robin boolean,
    server_tls_ca_file text,
    server_tls_cert_file text,
    server_tls_ciphers text,
    server_tls_key_file text,
    server_tls_protocols text,
    server_tls_sslmode text,
    stats_period int,
    stats_users text[],
    suspend_timeout interval,
    syslog boolean,
    syslog_facility text,
    syslog_ident text,
    tcp_defer_accept boolean,
    tcp_keepalive boolean,
    tcp_keepcnt int,
    tcp_keepidle int,
    tcp_keepintvl int,
    tcp_socket_buffer int,
    tcp_user_timeout int,
    track_extra_parameters text[],
    unix_socket_group text,
    unix_socket_mode int,
    "verbose" int
);

INSERT INTO pgbouncer.groups (name, peer_offset) VALUES ('default', 8);
INSERT INTO pgbouncer.settings (group_id) SELECT id FROM pgbouncer.groups WHERE name = 'default';
INSERT INTO pgbouncer.databases (group_id, name) SELECT id, '*' FROM pgbouncer.groups WHERE name = 'default';
INSERT INTO pgbouncer.users (group_id, name, max_user_connections) SELECT id, 'postgres', 123 FROM pgbouncer.groups WHERE name = 'default';
UPDATE pgbouncer.settings SET
    track_extra_parameters = ARRAY['IntervalStyle', 'search_path'],
    listen_addr = ARRAY['127.0.0.1'],
    listen_port = 6432,
    auth_type = 'trust',
    admin_users = ARRAY['jelte'],
    default_pool_size = 30,
    pool_mode = 'transaction',
    log_connections = true,
    log_disconnections = true,
    query_wait_timeout = '10s',
    max_prepared_statements = 100
WHERE group_id = (SELECT id FROM pgbouncer.groups WHERE name = 'default');

INSERT INTO pgbouncer.hba(
    group_id,
    position,
    type,
    database,
    user_name,
    address,
    method
) SELECT
    id,
    line_number,
    type,
    database,
    user_name,
    set_masklen(address::inet, masklen(netmask::inet))::inet::text,
    auth_method
FROM pgbouncer.groups, pg_hba_file_rules pghba
WHERE name = 'default'
    AND options IS NULL
    AND error IS NULL;

INSERT INTO pgbouncer.peers(
    group_id,
    peer_id,
    host
) SELECT
    id,
    1,
    'localhost'
FROM pgbouncer.groups
WHERE name = 'default';

INSERT INTO pgbouncer.auth (
    group_id,
    user_name,
    password
) SELECT id, usename, coalesce(passwd, '')
FROM pgbouncer.groups, pg_shadow
WHERE name = 'default';

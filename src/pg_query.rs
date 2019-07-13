extern crate config;

use std::string::String;
use std::result::Result;
use std::collections::HashMap;
use diesel::pg::PgConnection;
use diesel::sql_query;
use diesel::Connection;
use diesel::RunQueryDsl;
use std::result::Result::Ok;
use std::result::Result::Err;

use super::version::Version;
use super::host::Host;
use super::active_queries::ActiveQueries;
use super::sequential_scans::SequentialScans;

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub pg: HashMap<String, Host>
}

fn get_connection(pg_host: &Host) -> PgConnection {
    let pg_url = format!("postgres://{}:{}@{}:{}/{}", pg_host.username, pg_host.password, pg_host.host, pg_host.port, pg_host.dbname);
    PgConnection::establish(&pg_url)
        .expect(&format!("Error connecting to {}", pg_url))
}

impl PostgresConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();
        s.merge(config::File::with_name("/usr/local/postgrust/PGConfig")).unwrap();
        s.try_into()
    }

    pub fn test_connection(name: &String, pg_host: &Host) -> bool {
        println!("Connecting to {:?}", name);

        // get connection from connection pool
        let connection = self::get_connection(pg_host);

        let result = sql_query("SELECT version()")
            .load::<Version>(&connection);

        match result {
            Ok(v) => {
                let pg_version = &v[0];
                println!("Connected to Postgres. PG Version: {:?}", pg_version.version);
            },
            Err(e) => println!("error testing connection: {:?}", e),
        }

        true
    }

    pub fn active_queries(pg_host: &Host) {

        // get connection from connection pool
        let connection = self::get_connection(pg_host);

        let result = sql_query("SELECT datname, usename, client_addr, now() - query_start AS time_taken, query, pid, state FROM pg_stat_activity ac WHERE state = 'active' ORDER BY time_taken DESC; ")
            .get_result::<ActiveQueries>(&connection);

        println!("Result: {:?}", result);
    }

    pub fn sequential_scans(pg_host: &Host) {

        // get connection from connection pool
        let connection = self::get_connection(pg_host);

        let result = sql_query("SELECT relname, seq_scan FROM pg_stat_user_tables ORDER BY seq_scan DESC; ")
            .get_result::<SequentialScans>(&connection);

        println!("Result: {:?}", result);
    }
}
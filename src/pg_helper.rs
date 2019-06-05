extern crate config;
extern crate postgres;

use std::string::String;
use std::result::Result;
use std::collections::HashMap;
use self::postgres::Connection;
use self::postgres::TlsMode;

#[derive(Debug, Deserialize)]
pub struct Host {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub dbname: String
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub pg: HashMap<String, Host>
}

impl PostgresConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();
        s.merge(config::File::with_name("PGConfig")).unwrap();
        s.try_into()
    }

    //TODO: use connection pool from r2d2
    pub fn test_connection(name: &String, pg_host: &Host) -> bool {
        println!("Connecting to {:?}", name);

        let pg_url = format!("postgres://{}:{}@{}:{}/{}", pg_host.username, pg_host.password, pg_host.host, pg_host.port, pg_host.dbname);

        let conn = Connection::connect(pg_url, TlsMode::None).unwrap();
        for row in &conn.query("SELECT version()", &[]).unwrap() {
            println!("Found {:?}", row);
            println!("Connection to {:?} successful", name);
        }

        true
    }
}
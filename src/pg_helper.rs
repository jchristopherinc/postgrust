extern crate config;
extern crate postgres;

use std::string::String;
use std::result::Result;
use std::collections::HashMap;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;

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

type Pool = r2d2::Pool<PostgresConnectionManager>;

fn get_connection(pg_host: &Host) -> Pool {
    let pg_url = format!("postgres://{}:{}@{}:{}/{}", pg_host.username, pg_host.password, pg_host.host, pg_host.port, pg_host.dbname);
    let manager = PostgresConnectionManager::new(pg_url, TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    pool
}

impl PostgresConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();
        s.merge(config::File::with_name("PGConfig")).unwrap();
        s.try_into()
    }

    pub fn test_connection(name: &String, pg_host: &Host) -> bool {
        println!("Connecting to {:?}", name);

        // get connection from connection pool
        let client = self::get_connection(pg_host).get().unwrap();

        for row in &client.query("SELECT version()", &[]).unwrap() {
            let version: String = row.get("version");

            println!("Found {:?}", version);
            println!("Connection to {:?} successful", name);
        }

        true
    }
}
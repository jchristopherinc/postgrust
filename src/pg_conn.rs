extern crate config;

use std::string::String;
use std::result::Result;
use std::collections::HashMap;
use diesel::pg::PgConnection;
use diesel::sql_query;
use diesel::Connection;
use diesel::RunQueryDsl;

use diesel::sql_types::{Text};
use std::result::Result::Ok;
use std::result::Result::Err;
use std::vec::Vec;

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

#[derive(Debug, QueryableByName)]
pub struct Version {
    #[sql_type="Text"]
    pub version: String
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
                println!("Connecting to Postgres. PG Version: {:?}", pg_version.version)
            },
            Err(e) => println!("error testing connection: {:?}", e),
        }

        true
    }

    pub fn execute_and_print_result(pg_host: &Host, query: &str) {

    }
}
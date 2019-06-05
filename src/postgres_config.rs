extern crate config;

use std::string::String;
use std::result::Result;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Hosts {
    pub total: i32
}

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
    pub pg: HashMap<String, Host>,
    pub hosts: Hosts
}

impl PostgresConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();
        s.merge(config::File::with_name("PGConfig")).unwrap();
        s.try_into()
    }
}
extern crate config;

use std::string::String;
use std::result::Result;
use std::vec::Vec;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub authors: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct CargoConfig {
    pub package: Package
}

impl CargoConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();
        s.merge(config::File::with_name("Cargo")).unwrap();
        s.try_into()
    }
}
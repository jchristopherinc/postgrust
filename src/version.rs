use std::string::String;
use diesel::sql_types::{Text};

#[derive(Debug, QueryableByName)]
pub struct Version {
    #[sql_type="Text"]
    pub version: String
}
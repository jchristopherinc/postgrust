use std::string::String;

#[derive(Debug)]
pub struct Version {
    #[sql_type="Text"]
    pub version: String
}
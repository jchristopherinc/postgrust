use std::string::String;
use diesel::sql_types::{Text, Interval, Integer};
use diesel::pg::types::sql_types::Inet;
use ipnetwork::IpNetwork;
use diesel::pg::data_types::PgInterval;

/**
SELECT
  datname,
  usename,
  client_addr,
  now() - query_start AS time_taken,
  query,
  pid,
  state
FROM pg_stat_activity ac
WHERE state = 'active'
ORDER BY time_taken DESC;
*/

#[derive(Debug, QueryableByName)]
pub struct ActiveQueries {
    #[sql_type="Text"]
    pub datname: String,
    #[sql_type="Text"]
    pub usename: String,
    #[sql_type="Inet"]
    pub client_addr: IpNetwork,
    #[sql_type="Interval"]
    pub time_taken: PgInterval,
    #[sql_type="Text"]
    pub query: String,
    #[sql_type="Integer"]
    pub pid: i32,
    #[sql_type="Text"]
    pub state: String
}
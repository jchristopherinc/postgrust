use std::string::String;
use diesel::sql_types::{Text, BigInt};

/**
SELECT relname AS name,
       seq_scan as count
FROM
  pg_stat_user_tables
ORDER BY seq_scan DESC;
*/

#[derive(Debug, QueryableByName)]
pub struct SequentialScans {
    #[sql_type="Text"]
    pub relname: String,
    #[sql_type="BigInt"]
    pub seq_scan: i64
}
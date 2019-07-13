#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod cargo_config;
mod pg_conn;
mod host;
mod version;

use cargo_config::CargoConfig;
use pg_conn::PostgresConfig;

use clap::{Arg, App};
use std::string::String;

/*
 this enum values are lower_snake_cased as against Rust's convention of CamelCase, because for CLI snake case is much easier.
 Will keep it this way till I find a better approach.
 */
arg_enum!{
    #[derive(Debug)]
    enum Query {
        seq_scans,
        active_queries
    }
}

// casting non-text fields to text to circumvent type casting problem when parsing resultset.
fn get_matching_query(query_match: &Query) -> &str {
    match query_match {
        Query::active_queries => return "SELECT datname, usename, text(client_addr), text(now() - query_start) AS time_taken, query, text(pid), state FROM pg_stat_activity ac WHERE state = 'active' ORDER BY time_taken DESC;",
        Query::seq_scans => return "Select version();"
    }
}

fn main() {
    let config = CargoConfig::new().unwrap();
    let pg_config = PostgresConfig::new().unwrap();

    let matches = App::new(config.package.name)
        .version(&*config.package.version)
        .author(&*config.package.authors[0])
        .about("A PostgreSQL performance debug CLI <beta>")
        .arg(Arg::with_name("test")
            .short("-t")
            .long("--test")
            .default_value("true")
            .value_name("pg_test")
            .help("Tests if connection to PostgreSQL database(s) can be established"))
        .arg(Arg::with_name("query")
            .short("-q")
            .long("--query")
            .value_name("query")
            .takes_value(true)
            .help("Query to be executed")
            .possible_values(&Query::variants()))
        .get_matches_safe().unwrap_or_else(|e| e.exit());


    // connection testing
    let test = matches.value_of("test").unwrap();
    if test == "true" {
        println!("Initiating PG connection test to all physical hosts");
        for (name, pg_host) in &pg_config.pg {
            PostgresConfig::test_connection(&name, &pg_host);
        }
    }


    if matches.occurrences_of("query") > 0 {

        let matching_query_arg = value_t!(matches.value_of("query"), Query).unwrap_or_else(|e| e.exit());
        let matching_query = self::get_matching_query(&matching_query_arg);

        for (_name, pg_host) in &pg_config.pg {
            PostgresConfig::execute_and_print_result(&pg_host, &matching_query)
        }
    }
}

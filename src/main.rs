#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate diesel;
extern crate ipnetwork;
extern crate chrono;

mod cargo_config;
mod pg_query;
mod host;
mod version;
mod active_queries;

use cargo_config::CargoConfig;
use pg_query::PostgresConfig;

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

        for (_name, pg_host) in &pg_config.pg {
            match matching_query_arg {
                Query::active_queries => {
                    PostgresConfig::active_queries(&pg_host);
                },
                Query::seq_scans => {
                    PostgresConfig::active_queries(&pg_host); //TODO: fix it once active_queries work
                }
            }
        }
    }
}

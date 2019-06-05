#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_postgres;

mod cargo_config;
mod pg_helper;

use cargo_config::CargoConfig;
use pg_helper::PostgresConfig;

use clap::{Arg, App};

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


    let query_match = value_t!(matches.value_of("query"), Query).unwrap_or_else(|e| e.exit());

    //should change it to hashmap
    let dummy_query = "SELECT
                              datname,
                              usename,
                              client_addr,
                              now() - query_start AS time_taken,
                              query,
                              pid,
                              state
                            FROM pg_stat_activity ac
                            WHERE state = 'active'
                            ORDER BY time_taken DESC;";

    for (name, pg_host) in &pg_config.pg {
        match query_match {
            Query::active_queries => PostgresConfig::execute_and_print_result(&pg_host, &dummy_query),
            Query::seq_scans => println!("Sequential Scans")
        }
    }
}

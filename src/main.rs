extern crate clap;
#[macro_use]
extern crate serde_derive;

mod cargo_config;
mod postgres_config;

use cargo_config::CargoConfig;
use postgres_config::PostgresConfig;

use clap::{Arg, App};

fn main() {
    let config = CargoConfig::new().unwrap();
    let pg_config = PostgresConfig::new().unwrap();

    let matches = App::new(config.package.name)
        .version(&*config.package.version)
        .author(&*config.package.authors[0])
        .about("A PostgreSQL performance debug CLI")
        .arg(Arg::with_name("test")
            .short("-t")
            .long("--test")
            .default_value("true")
            .value_name("pg_test")
            .help("Tests if connection to PostgreSQL database can be established"))
        .get_matches_safe().unwrap_or_else(|e| e.exit());
    let test = matches.value_of("test").unwrap();

    if test == "true" {
        println!("{}", test);
    }

    println!("PG DB");
    println!("{:?}", pg_config.hosts.total);

    for (_name, pg_host) in pg_config.pg {
        println!("{:?}", pg_host.host)
    }
}

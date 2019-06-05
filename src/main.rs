extern crate clap;
#[macro_use]
extern crate serde_derive;

mod cargoconfig;

use cargoconfig::CargoConfig;
use clap::{Arg, App};

fn main() {
    let config = CargoConfig::new().unwrap();

    let matches = App::new(config.package.name)
        .version(&*config.package.version)
        .author(&*config.package.authors[0])
        .about("A PostgreSQL debug CLI")
        .arg(Arg::with_name("test")
            .short("-t")
            .required(true)
            .takes_value(true)
            .help("test"))
        .get_matches_safe().unwrap_or_else(|e| e.exit());
    let test = matches.value_of("test").unwrap();
    println!("{}", test);
}

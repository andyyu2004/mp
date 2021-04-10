use clap::load_yaml;
use clap::{App, ArgMatches};

pub(crate) fn get_args() -> ArgMatches {
    let yaml = load_yaml!("cli.yaml");
    App::from(yaml).get_matches()
}

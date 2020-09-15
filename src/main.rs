use std::fs::File;
use std::io::Read;

use argparse::{ArgumentParser, Store};

mod config;
use crate::config::Config;

fn main() {
    let Options {
        config_filename,
        seed,
    } = parse_options();

    let config_text = {
        let mut file = match File::open(config_filename) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("error: {}", error);
                std::process::exit(1);
            }
        };

        let mut text = String::new();

        if let Err(error) = file.read_to_string(&mut text) {
            eprintln!("error: {}", error);
            std::process::exit(1);
        }

        text
    };

    let config: Config = match json5::from_str(&config_text) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("error: {}", error);
            std::process::exit(1);
        }
    };

    println!("{}", seed);
    println!("{:?}", config);
}

struct Options {
    config_filename: String,
    seed: u64,
}

fn parse_options() -> Options {
    let mut options = Options {
        config_filename: String::new(),
        seed: 0,
    };

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("Simulate interacting state machines.");

        parser.refer(&mut options.seed).add_option(
            &["-s", "--seed"],
            Store,
            "Specify seed value for random number generation",
        );

        parser
            .refer(&mut options.config_filename)
            .add_argument(
                "config",
                Store,
                "Configuration file defining the network of state machines",
            )
            .required();

        parser.parse_args_or_exit();
    }

    return options;
}

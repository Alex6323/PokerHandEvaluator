use phevor;
use phevor::display::*;
use phevor::eval::*;
use phevor::models::*;

use clap::load_yaml;
use clap::App;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let hands = if matches.is_present("file") {
        create_hands_from_file("hands.txt")
    } else {
        if matches.is_present("hand") {
            vec![String::from(
                matches
                    .value_of("hand")
                    .unwrap_or_else(|| panic!("error: cannot read hand")),
            )]
        } else {
            vec![]
        }
    };

    println!("evaluations:");
    print_all_evals(&hands);
    println!();
    println!("ranking:");
    print_all_hands_ranked(&hands);
}

pub fn create_hands_from_file(file: &str) -> Vec<String> {
    let mut hands = vec![];
    let buffered = BufReader::new(File::open(file).expect("File does not exist."));

    buffered
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            hands.push(line);
        });

    hands
}

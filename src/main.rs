use phevor::display::*;
use phevor::file::*;

use clap::load_yaml;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let hands = if matches.is_present("file") {
        create_hands_from_file("hands.txt")
    } else if matches.is_present("hand") {
        vec![String::from(
            matches
                .value_of("hand")
                .unwrap_or_else(|| panic!("error: cannot read hand")),
        )]
    } else {
        vec![]
    };

    if !matches.is_present("rank") {
        println!("evaluations:\n============");
        print_all_evals(&hands);
        println!();
    }

    if matches.is_present("all") || matches.is_present("rank") {
        println!("ranking:\n========");
        print_all_hands_ranked(&hands);
    }
}

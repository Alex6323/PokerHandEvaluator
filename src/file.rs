use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
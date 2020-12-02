use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufReader,BufRead};

pub fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()
}

pub fn read_file_lines(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines : Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    lines
}

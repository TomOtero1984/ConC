use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::charset::SymbolMap;

pub fn encode_file(input_path: String, output_path: String, map: &SymbolMap) {
    let input_file = File::open(&input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(&output_path).expect("Failed to create output file");

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let encoded_line = encode_line(&line, map);
        writeln!(output_file, "{}", encoded_line).expect("Failed to write line");
    }
}

fn encode_line(line: &str, map: &SymbolMap) -> String {
    line.split_whitespace()
        .map(|word| map.lookup(word).unwrap_or(word).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
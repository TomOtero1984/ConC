// src/decoder.rs

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::charset::SymbolMap;

pub fn decode_file(input_path: String, output_path: String, map: &SymbolMap) {
    let input_file = File::open(&input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(&output_path).expect("Failed to create output file");

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let decoded_line = decode_line(&line, map);
        writeln!(output_file, "{}", decoded_line).expect("Failed to write line");
    }
}

fn decode_line(line: &str, map: &SymbolMap) -> String {
    line.split_whitespace()
        .map(|sym| map.reverse_lookup(sym).unwrap_or(sym).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
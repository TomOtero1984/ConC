use crate::data_handler::io;
use crate::lexicon::word_map::Index;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

static CHARSET: [&str; 4096] = charset!();

pub fn generate_word_map_jsonl(input_path: &str, output_path: &str) {
    let reader = io::make_buf_reader(input_path).unwrap();
    let mut writer = io::make_buf_writer(output_path).unwrap();

    let mut index: Index = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let base_word = convert_to_base4096(index);
            let json = json!({
                "index": index,
                "natural": line.trim(),
                "conc": base_word,
            });

            if let Err(e) = serde_json::from_value::<ConCEntry>(json.clone()) {
                eprintln!("Skipping invalid entry: {} | Error: {}", line.trim(), e);
                continue;
            }

            writeln!(writer, "{}", json.to_string()).expect("Failed to write line");
            index += 1;
        }
    }
    writer.flush().expect("Failed to flush output");
}

pub fn convert_to_base4096(num: Index) -> String {
    let base = 4096;
    let low_idx = num % base;
    let high_idx = num / base;

    let low_char = CHARSET[low_idx as usize];
    let high_char = CHARSET[high_idx as usize];

    format!("{}{}", high_char, low_char)
}

pub fn convert_from_base4096(word: &str) -> Index {
    let low_char = word.chars().nth(0).unwrap();
    let high_char = word.chars().nth(1).unwrap();
    todo!();
    0 // [TODO] Update this to be... an actual function...
}

#[derive(Debug, Deserialize)]
pub struct ConCEntry {
    pub index: u32,
    pub natural: String,
    pub conc: String,
}

pub fn validate_jsonl(path: &str) {
    let file = File::open(path).expect("Failed to open JSONL file");
    let reader = BufReader::new(file);

    let mut line_num = 1;
    for line in reader.lines() {
        match line {
            Ok(ref l) => match serde_json::from_str::<ConCEntry>(l) {
                Ok(_) => {} // Valid
                Err(e) => {
                    eprintln!(
                        "Line {}: Invalid JSON entry\n  {}\n  Error: {}",
                        line_num, l, e
                    );
                }
            },
            Err(e) => {
                eprintln!("Line {}: Failed to read line: {}", line_num, e);
            }
        }
        line_num += 1;
    }
}


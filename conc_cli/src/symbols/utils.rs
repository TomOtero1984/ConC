use serde_json::json;
use serde::Deserialize;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use crate::SymbolMap;



static CHARSET: [&str; 4096] = charset!();

pub fn generate_word_map(input_path: &str, output_path: Option<&str>) {
    let input_file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);
    
    let stdout = std::io::stdout();
    let mut writer: Box<dyn Write> = match output_path {
        Some(path) => Box::new(BufWriter::new(File::create(path).expect("Failed to create output file"))),
        None => Box::new(stdout.lock()),
    };

    let mut index: u64 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let base_word = convert_to_base4096(index);
            let default_symbol = CHARSET[0];

            let json = json!({
                "word": line.trim(),
                "base_word": base_word,
                "tone": "neutral",
                "base_tone": default_symbol,
                "presentation": "lowercase",
                "base_presentation": default_symbol
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


pub fn convert_to_base4096(num: u64) -> String {
    let base = 4096;
    let low_idx = num % base;
    let high_idx = num / base;
    
    let low_char = CHARSET[low_idx as usize];
    let high_char = CHARSET[high_idx as usize];

    format!("{}{}", high_char, low_char)
}




#[derive(Debug, Deserialize)]
pub struct ConCEntry {
    pub word: String,
    pub base_word: String,
    pub tone: String,
    pub base_tone: String,
    pub presentation: String,
    pub base_presentation: String,

    #[serde(default)]
    pub zone: Option<String>,
    #[serde(default)]
    pub synonyms: Option<Vec<String>>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
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
                    eprintln!("Line {}: Invalid JSON entry\n  {}\n  Error: {}", line_num, l, e);
                }
            },
            Err(e) => {
                eprintln!("Line {}: Failed to read line: {}", line_num, e);
            }
        }
        line_num += 1;
    }
}
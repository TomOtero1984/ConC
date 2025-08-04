use crate::lexicon::utils::{ConCEntry, convert_to_base4096};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::vec::Vec;

pub type WordVector = Vec<String>;
pub type WordHash = HashMap<String, String>;

pub struct WordMap {
    word_vec: WordVector,
    word_hash: WordHash,
}
impl WordMap {

    pub fn new() -> Self {
        Self {
            word_vec: WordVector::new(),
            word_hash: WordHash::new(),
        }
    }
}


pub fn generate_word_map(input_path: &str, output_path: Option<&str>) {
    // Load stream
    // Make reader
    // Set output
    // Make Writer


//--CUT--//
    let input_file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    let stdout = std::io::stdout();
    let mut writer: Box<dyn Write> = match output_path {
        Some(path) => Box::new(BufWriter::new(
            File::create(path).expect("Failed to create output file"),
        )),
        None => Box::new(stdout.lock()),
    };
//--//

    let mut index: u32 = 0;
    let mut word_vec = WordVector::new();
    let mut word_hash = WordHash::new();

    for line in reader.lines() {
        if let Ok(line) = line {

            let natural = line.trim().to_string();
            let conc_word = convert_to_base4096(index);
            if json!({
                "index": index,
                "natural": natural,
                "conc": conc_word,
            })["success"] != true {
                eprintln!("Skipping invalid entry: {}", line.trim());
                continue;
            }

            word_vec.push(natural.clone());
            word_hash.insert(natural.clone(), conc_word);

            index += 1;
        }
    }
    writer.flush().expect("Failed to flush output");
}






// pub fn generate_word_map(input_path: &str, output_path: Option<&str>) {
//     let input_file = File::open(input_path).expect("Failed to open input file");
//     let reader = BufReader::new(input_file);
//
//     let stdout = std::io::stdout();
//     let mut writer: Box<dyn Write> = match output_path {
//         Some(path) => Box::new(BufWriter::new(
//             File::create(path).expect("Failed to create output file"),
//         )),
//         None => Box::new(stdout.lock()),
//     };
//
//     let mut index: u32 = 0;
//     let mut word_vec = WordVector::new();
//     let mut word_hash = WordHash::new();
//
//     for line in reader.lines() {
//         if let Ok(line) = line {
//
//             let natural = line.trim().to_string();
//             let conc_word = convert_to_base4096(index);
//             if json!({
//                 "index": index,
//                 "natural": natural,
//                 "conc": conc_word,
//             })["success"] != true {
//                 eprintln!("Skipping invalid entry: {}", line.trim());
//                 continue;
//             }
//
//             word_vec.push(natural.clone());
//             word_hash.insert(natural.clone(), conc_word);
//
//             index += 1;
//         }
//     }
//     writer.flush().expect("Failed to flush output");
// }

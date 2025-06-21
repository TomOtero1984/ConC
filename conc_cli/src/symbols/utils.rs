use std::io::{BufRead, BufReader, Write};
use crate::SymbolMap;

pub fn generate_word_map(input_path: &str) {
    use std::fs::File as StdFile;
    let chunk = StdFile::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(chunk);

    reader.lines().for_each(|line| {
        match line {
            Ok(line) => {
                
            }
            Err(e) => {
                let _ = std::io::stdout().write_all(&format!("{}\n", e.to_string()).into_bytes());
            }
        }
    })


}

pub fn convert_to_base4096(num: u64) -> String {
    let res = String::new();
    // 1. divide
    // 2. multiply remainder by base, round down
    // 3. look up quotient and value of step 2
    //    in charset
    let base = 4096;
    let low_idx = num % base;
    let high_idx = num / base;


    let charset = charset!();
    let low_char = charset[low_idx as usize];
    let high_char = charset[high_idx as usize];

    return res;

}
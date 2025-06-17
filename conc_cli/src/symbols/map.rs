use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Entry {
    word: String,
    symbol: String,
}

pub struct SymbolMap {
    map: HashMap<String, String>,
}

impl SymbolMap {
    pub fn from_jsonl_files(paths: &[&str]) -> Self {
        let mut map = HashMap::new();
        
        for path in paths {
            let file = File::open(path).expect("Failed to open symbol map file");
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(entry) = serde_json::from_str::<Entry>(&line) {
                        map.insert(entry.word, entry.symbol.clone());
                    }
                }
            }
        }
        SymbolMap { map }
    }
    pub fn lookup(&self, word: &str) -> Option<&str> {
        self.map.get(&word.to_lowercase()).map(|s| s.as_str())
    }
    pub fn reverse_lookup(&self, symbol: &str) -> Option<&str> {
        // Build a reverse lookup only once
        self.map.iter()
            .find_map(|(word, sym)| if sym == symbol { Some(word.as_str()) } else { None })
    } 
}
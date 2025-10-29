use crate::data_handler::io::read_lines;
use crate::data_handler::schema::WordSchema;
use crate::lexicon::word_map::{ConcWord, Index, NaturalWord};

pub fn read_word_list(path: &str) -> Result<Vec<String>, std::io::Error>{
    let word_list = read_lines(path)?;
    word_list.collect()
}



pub fn format_json(index: Index, conc_word: &ConcWord, natural_word: &NaturalWord)
    -> Result<String, std::io::Error>{
    let fmt_json: WordSchema = WordSchema {
        index: index as usize,
        natural: natural_word.text.to_string(),
        conc: conc_word.text.to_string(),
    };
    Ok(serde_json::to_string_pretty(&fmt_json)?)
}

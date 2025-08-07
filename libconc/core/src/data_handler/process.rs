use crate::data_handler::io::read_lines;

pub fn read_word_list(path: &str) -> Result<Vec<String>, std::io::Error>{
    let word_list = read_lines(path)?;
    word_list.collect()
}
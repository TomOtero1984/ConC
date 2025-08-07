use std::io::Write;
use conc_core::lexicon::utils;
use conc_core::lexicon::word_map::WordMap;
use conc_core::data_handler::process;

#[test]
fn test_generate_word_map_jsonl () {
    let input_path = "tests/dict/natural_english_words.txt";
    let output_path = "tests/output/test_word_map.jsonl";

    // Ensure clean test path
    if std::path::Path::new(output_path).exists() {
        std::fs::remove_file(output_path).unwrap();
    }

    utils::generate_word_map_jsonl(input_path, output_path);
    assert!(std::path::Path::new(output_path).exists());
}



fn make_wmap() -> WordMap {
    let input_path = "tests/dict/natural_english_words.txt";
    let mut wmap = WordMap::new();
    wmap.generate(&process::read_word_list(input_path).unwrap());
    wmap
}
#[test]
fn test_word_map() {
    let wmap = make_wmap();
    assert!(wmap.get_conc_word(29999).is_some());
}

#[test]
fn test_word_map_conc() {
    let wmap = make_wmap();
    let mut file = std::fs::File::create("tests/output/test_wmap_conc.txt").unwrap();
    for word in wmap.conc_hash.iter() {
        let fmt_msg = format!("index: {}, conc: {}", word.1, word.0  );
        file.write(fmt_msg.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}

#[test]
fn test_word_map_natural() {
    let wmap = make_wmap();
    let mut file = std::fs::File::create("tests/output/test_wmap_natural.txt").unwrap();
    for word in wmap.natural_hash.iter() {
        let fmt_msg = format!("index: {}, natural: {}", word.1, word.0  );
        file.write(fmt_msg.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
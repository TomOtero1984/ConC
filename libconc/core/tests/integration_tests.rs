use conc_core::lexicon::utils;

#[test]
fn test_generate_word_map () {
    let input_path = "tests/dict/native_english_words.txt";
    let output_path = "tests/output/test_word_map.jsonl";

    // Ensure clean test path
    if std::path::Path::new(output_path).exists() {
        std::fs::remove_file(output_path).unwrap();
    }

    utils::generate_word_map(input_path, output_path);
    assert!(std::path::Path::new(output_path).exists());

}
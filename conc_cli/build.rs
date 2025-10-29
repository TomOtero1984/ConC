// build.rs
use std::fs;
use std::path::Path;

fn main() {
    // Path to your text file
    let input_path = Path::new("dict/words.txt");

    // Read the file contents
    let contents = fs::read_to_string(input_path).expect("Failed to read input file");

    // Generate a Rust source file in the OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("words.rs");

    // Write it as a static string
    fs::write(
        &dest_path,
        format!("pub const WORD_LIST: &str = r#\"{}\"#;", contents),
    )
    .expect("Failed to write output file");
}

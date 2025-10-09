use clap::{Parser, Subcommand};
use conc_core::data_handler::schema::WordSchema;
use conc_core::data_handler::{io, process};
use conc_core::lexicon::word_map::{Index, WordMap};
use std::io::BufRead;

#[derive(Parser)]
#[command(
    name = "conc",
    about = "ConC - Compress and decompress English using symbolic encoding"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SearchByIndex { index: Index },
    SearchByWord { word: String, word_type: String },
}

fn get_word(wmap: &WordMap, index: Index) {
    let word = wmap.word_hash.get(&index).unwrap();
    let conc_word = &word.conc;
    let natural = &word.natural;
    let fmt_out = process::format_json(index.clone(), conc_word, natural);
    io::write_lines("-", &[fmt_out.unwrap()]);
}

include!(concat!(env!("OUT_DIR"), "/words.rs"));
fn main() {
    let cli = Cli::parse();
    let mut wmap = WordMap::new();
    let cursor = std::io::Cursor::new(WORD_LIST.as_bytes());
    let reader = std::io::BufReader::new(cursor);
    let words = reader.lines().collect::<Result<Vec<_>, _>>();
    wmap.generate(&words.unwrap());

    // let input_path = include_str!("../dict/native_english_words.txt");

    // let word_list = &process::read_word_list(input_path).unwrap();
    // wmap.generate();

    match cli.command {
        Commands::SearchByIndex { index } => {
            get_word(&wmap, index);
        }

        Commands::SearchByWord { word, word_type } => {
            let maybe_index = match word_type.as_str() {
                "conc" => {
                    println!("Searching by conc word");
                    wmap.conc_hash.get(&word).copied()
                }
                "natural" => {
                    println!("Searching by natural word");
                    wmap.natural_hash.get(&word).copied()
                }
                _ => {
                    println!("Unknown word type");
                    None
                }
            };
            if let Some(index) = maybe_index {
                get_word(&wmap, index);
            }
        }
    }
}

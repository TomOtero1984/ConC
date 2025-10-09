use clap::{Parser, Subcommand, ValueEnum};
use conc_core::data_handler::schema::WordSchema;
use conc_core::data_handler::{io, process};
use conc_core::lexicon::word_map::{Index, WordMap};
use std::io::BufRead;

#[derive(Parser)]
#[command(
    author = "Tom Otero",
    version = "0.0.1",
    about = "ConC - Compress and decompress English using symbolic encoding"
)]
struct Cli {
    /// Search by index (mutually exclusive with -w)
    #[arg(short = 'i', long = "index")]
    index: Option<usize>,

    /// Search by word (mutually exclusive with -i)
    #[arg(short = 'w', long = "word")]
    word: Option<String>,

    /// Word type: conc or natural
    #[arg(short = 't', long = "type", value_enum, default_value_t = WordType::Conc)]
    word_type: WordType,

    // #[command(subcommand)]
    // command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SearchByIndex { index: Index },
    SearchByWord { word: String, word_type: String },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum WordType {
    Conc,
    Natural,
}

fn get_word(wmap: &WordMap, index: Index) -> Result<(), Box<dyn std::error::Error>> {
    let word = wmap
        .word_hash
        .get(&index)
        .ok_or_else(|| format!("Index {} is not found.", index))?;
    let conc_word = &word.conc;
    let natural = &word.natural;

    let fmt_out = process::format_json(index.clone(), conc_word, natural)?;
    io::write_lines("-", &[fmt_out])?; // propagate write error too
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/words.rs"));
fn main() {
    let cli = Cli::parse();

    let mut wmap = WordMap::new();
    let cursor = std::io::Cursor::new(WORD_LIST.as_bytes());
    let reader = std::io::BufReader::new(cursor);
    let words = reader.lines().collect::<Result<Vec<_>, _>>();
    wmap.generate(&words.unwrap());

    match (cli.index, cli.word) {
        (Some(index), None) => {
            if let Err(e) = get_word_by_index(&wmap, index) {
                eprintln!("{e}");
            }
        }
        (None, Some(word)) => {

            if let Err(e) = get_word_by_word(&wmap, &word.to_lowercase(), cli.word_type) {
                eprintln!("{e}");
            }
        }
        (Some(_), Some(_)) => {
            eprintln!("Please provide either -i/--index or -w/--word, not both.");
            std::process::exit(2);
        }
        (None, None) => {
            eprintln!("Please provide one of: -i INDEX  or  -w WORD  (try --help).");
            std::process::exit(2);
        }
    }

    // match cli.command {
    //     Commands::SearchByIndex { index } => {
    //         get_word(&wmap, index).unwrap_or_else(|_e| eprintln!("Index {} not found.", index));
    //     }

    //     Commands::SearchByWord { word, word_type } => {
    //         let maybe_index: Result<usize, String> = match word_type.as_str() {
    //             "conc" => {
    //                 println!("Searching by conc word");
    //                 wmap.conc_hash
    //                     .get(&word)
    //                     .copied() // Option<&usize> → Option<usize>
    //                     .ok_or_else(|| format!("Word {} not found.", word))
    //             }
    //             "natural" => {
    //                 println!("Searching by natural word");
    //                 wmap.natural_hash
    //                     .get(&word)
    //                     .copied()
    //                     .ok_or_else(|| format!("Word {} not found.", word))
    //             }
    //             _ => Err(format!("Unknown word type: {}", word_type)),
    //         };

    //         match maybe_index {
    //             Ok(index) => {
    //                 get_word(&wmap, index)
    //                     .unwrap_or_else(|_e| eprintln!("Word {} not found.", word));
    //             }
    //             Err(e) => eprintln!("{}", e),
    //         }
    //     }
    // }
}

fn get_word_by_index(wmap: &WordMap, index: usize) -> Result<(), String> {
    get_word(&wmap, index).unwrap_or_else(|_e| eprintln!("Index {} not found.", index));
    Ok(())
}

fn get_word_by_word(wmap: &WordMap, word: &str, word_type: WordType) -> Result<(), String> {
    let maybe_index: Result<usize, String> = match word_type {
        WordType::Conc => {
            println!("Searching by conc word");
            wmap.conc_hash
                .get(word)
                .copied() // Option<&usize> → Option<usize>
                .ok_or_else(|| format!("Word {} not found.", word))
        }
        WordType::Natural => {
            println!("Searching by natural word");
            wmap.natural_hash
                .get(word)
                .copied()
                .ok_or_else(|| format!("Word {} not found.", word))
        }
        _ => Err(format!("Unknown word type")),
    };

    match maybe_index {
        Ok(index) => {
            get_word(&wmap, index).unwrap_or_else(|_e| eprintln!("Word {} not found using index{}.", word, index));
        }
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

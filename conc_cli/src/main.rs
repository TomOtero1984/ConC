use clap::{Parser, Subcommand};
use conc_core::data_handler::schema::WordSchema;
use conc_core::data_handler::{io, process};
use conc_core::lexicon::word_map::{Index, WordMap};

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
    HelloWorld {},
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

fn main() {
    let cli = Cli::parse();
    let input_path = "dict/native_english_words.txt";
    let mut wmap = WordMap::new();
    let word_list = &process::read_word_list(input_path).unwrap();
    wmap.generate(word_list);

    match cli.command {
        Commands::HelloWorld {} => {
            let hello = wmap.natural_hash.get("hello").unwrap();
            let world = wmap.natural_hash.get("world").unwrap();
            println!("hello: {}, world: {}", hello, world);

            let conc_hello = wmap.conc_hash.get("Ĕ瑿").unwrap();
            let conc_world = wmap.conc_hash.get("ĹŃ").unwrap();
            println!(
                "conc_hello [Ĕ瑿]: {}, conc_world [ĹŃ]: {}",
                conc_hello, conc_world
            );

            let wm_hello = wmap.word_hash.get(hello).unwrap();
            let wm_world = wmap.word_hash.get(world).unwrap();
            println!(
                "wm_hello: {}, wm_world: {}",
                wm_hello.conc.text, wm_world.conc.text
            );
            println!(
                "wm_hello_index: {}, wm_world_index: {}",
                wm_hello.conc.index, wm_world.conc.index
            );
        }
        Commands::SearchByIndex { index} => {
            get_word(wmap, index);
        }

        Commands::SearchByWord { word, word_type } => {
           let maybe_index = match word_type.as_str() {
                "conc" => {
                    println!("Searching by conc word");
                    wmap.conc_hash.get(&word).copied()

                },
                "natural" => {
                    println!("Searching by natural word");
                    wmap.natural_hash.get(&word).copied()
                },
                _ => {
                    println!("Unknown word type");
                    None
                }
            };
           if let Some(index) = maybe_index {
               get_word(wmap, index);
           }
        }
    }
}

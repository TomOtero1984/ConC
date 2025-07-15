use std::fs;
use clap::{Parser, Subcommand};
use conc_cli::{encoder, decoder, SymbolMap, symbols};
#[derive(Parser)]
#[command(
    name = "conc",
    about = "ConC.GPT - Compress and decompress English using symbolic encoding"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,

        #[arg(short = 'm', long)]
        map_dir: String,
    },
    Decode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,

        #[arg(short = 'm', long)]
        map_dir: String,
    },
    GenerateWordMap {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long)]
        output: Option<String>,
    },
    ValidateJsonl {
        #[arg(short, long)]
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input, output, map_dir } => {
            println!("Encoding: {} -> {}", input, output);
            let files: Vec<String> = fs::read_dir(map_dir)
                .expect("Failed to read map directory")
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            Some(path.to_string_lossy().into_owned())
                        } else {
                            None
                        }
                    })
                })
                .collect();
            let map_paths = files.iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let map = SymbolMap::from_jsonl_files(&map_paths);
            encoder::encode_file(input.clone(), output.clone(), &map);
        }
        Commands::Decode { input, output, map_dir } => {
            println!("Decoding: {} -> {}", input, output);
            let files: Vec<String> = fs::read_dir(map_dir)
                .expect("Failed to read map directory")
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            Some(path.to_string_lossy().into_owned())
                        } else {
                            None
                        }
                    })
                })
                .collect();

            let map_paths = files.iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let map = SymbolMap::from_jsonl_files(&map_paths);
            decoder::decode_file(input, output, &map);
        }
        Commands::GenerateWordMap { input, output  } => {
            println!("Generating ConC word map from: {}", input);
            println!("Saving to: {}", output.as_deref().unwrap_or("None (stdout)"));
            symbols::utils::generate_word_map(&input, output.as_deref());
        }
        Commands::ValidateJsonl { input } => {
            println!("Validating: {}", input);
            symbols::utils::validate_jsonl(&input);
        }
    }
}

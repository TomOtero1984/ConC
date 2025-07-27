use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::collections::HashMap;

use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct Word {
    index: u16,
    word: String,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}



pub fn translate(word: &str) -> String {
    format!("Translated: {}", word)
}





pub mod wasm;
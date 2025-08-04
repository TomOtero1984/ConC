use std::fs;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

pub fn read_input(path: &str) -> io::Result<String> {
    if path == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        fs::read_to_string(path)
    }
}

pub fn write_output(path: &str, content: &str) -> io::Result<()> {
    if path == "-" {
        io::stdout().write_all(content.as_bytes())
    } else {
        fs::write(path, content)
    }
}

pub fn make_reader(path: &str) -> io::Result<Box<dyn Read>> {
    if path == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        Ok(Box::new(File::open(path)?))
    }
}

pub fn make_writer(path: &str) -> io::Result<Box<dyn Write>> {
    if path == "-" {
        Ok(Box::new(io::stdout()))
    } else {
        Ok(Box::new(File::create(path)?))
    }
}

pub fn make_buf_reader(input_path: &str) -> io::Result<BufReader<File>>{
    let input_file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);
    Ok(reader)
}

pub fn make_buf_writer(output_path: &str) -> io::Result<BufWriter<File>>{
    let output_file = File::create(output_path).expect("Failed to create output file");
    let writer = BufWriter::new(output_file);
    Ok(writer)
}

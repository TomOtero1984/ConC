use std::fs;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, BufRead, Read, Write};

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

pub fn read_lines(path: &str)
                         -> io::Result<impl Iterator<Item = io::Result<String>>> {

    let reader: Box<dyn Read> = if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(path)?)
    };

    Ok(BufReader::new(reader).lines())
}

pub fn write_lines<I, S>(path: &str, lines: I) -> io::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let writer: Box<dyn Write> = if path == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(path)?)
    };

    let mut writer = BufWriter::new(writer);

    for line in lines {
        writeln!(writer, "{}", line.as_ref())?;
    }

    writer.flush()?;
    Ok(())
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

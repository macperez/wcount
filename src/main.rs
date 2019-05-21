use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use structopt::StructOpt;
use ::wcount::*;



/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    
    #[structopt(short = "l", long = "lines")]
    lines: bool,
    #[structopt(short = "w", long = "words")]
    words: bool,
    #[structopt(short = "b", long = "bytes")]
    bytes: bool,
}

#[derive(Debug)]
struct CustomError(String);


// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), CustomError> {
    let args = Cli::from_args();

    // Pequeña versión de la herramienta de UNIX wc

    let display = &args.path.display();
    let mut file = File::open(&args.path).map_err(|err| CustomError(format!("Error reading `{}`: {}", display, err)))?;
    let mut parameters = false;
    if args.lines{
        parameters = true;
        println!("{} {}", count_lines(&file), display);
        file.seek(SeekFrom::Start(0)).unwrap();    
    }

    if args.bytes{
        parameters = true;
        println!("{} {}", count_bytes(&file), display);
        file.seek(SeekFrom::Start(0)).unwrap();    
    }

    if args.words{
        parameters = true;
        println!("{} {}", count_words(&file), display);
        file.seek(SeekFrom::Start(0)).unwrap();
    }

    if !parameters{
        let lines = count_lines(&file);
        file.seek(SeekFrom::Start(0)).unwrap();    
        let words = count_words(&file);
        file.seek(SeekFrom::Start(0)).unwrap();    
        let bytes = count_bytes(&file);
        file.seek(SeekFrom::Start(0)).unwrap();    
        println!("{} {} {} {} ", lines, words, bytes, display);
    }
    Ok(())
}



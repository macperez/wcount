use std::io::{BufReader,BufRead};
use std::fs::File;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    // Pequeña versión de la herramienta de UNIX wc
    // let filename = "/home/manu/projects/data/sample_fasta_files/sam/5867-EA20-wolf-feces-Qiagen-17usat-10-10_S13_L001_R_001_u225_AGCGACTATTATATGCCAGCG.sam";
    let file = File::open(&args.path)?;
    //let filename = file.metadata();
    // println!("Reading {}", filename);
    println!("Number of lines {} ", count_lines(&file));
    println!("Number of words {} ", count_words(&file));
    Ok(())
}


fn count_lines(file: &File)-> i32 {
    let mut n_lines = 0;
    for _line in BufReader::new(file).lines() {
        n_lines += 1;
    }
    n_lines
}

fn count_words(file: &File)-> i32 {
    let mut n_words = 0;
    // assert_eq!(lines_iter.next(), Some(String::from("lorem")));
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        for _word in line.split_whitespace(){
            n_words += 1;
        }

    }
    n_words
}

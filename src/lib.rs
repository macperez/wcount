use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;



pub fn count_lines(file: &File)-> i32 {
    let mut n_lines = 0;
    for _line in BufReader::new(file).lines() {
        n_lines += 1;
    }
    n_lines

}

pub fn count_words(file: &File)-> i32 {
    let mut n_words = 0;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        for _word in line.split_whitespace(){
            n_words += 1;
        }
    }
    n_words
}

pub fn count_bytes(file: &File)-> usize {
    let mut n_bytes = 0;
    for line in BufReader::new(file).lines(){
        let line = line.unwrap();
        n_bytes += line.len();
    }

    n_bytes
}

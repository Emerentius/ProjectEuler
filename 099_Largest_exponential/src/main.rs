#![feature(test)]
extern crate test;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut file = match File::open(Path::new("./p099_base_exp.txt")) {
        Err(_) => panic!("couldn't open file"),
        Ok(file) => file,
    };
    let mut string = String::new();
    file.read_to_string(&mut string);

    let mut max_line = 0;
    let mut max_log = 0.;
    for (line_nr, line) in string.lines().enumerate() {
        let base_exp : Vec<_> = line.split(",")
            .map(|s| s.parse::<f64>().unwrap() )
            .collect();
        let log_num = base_exp[0].log2() * base_exp[1];

        if log_num > max_log {
            max_line = line_nr;
            max_log = log_num;
        }
    }
    println!("{}",max_line+1);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main());
}

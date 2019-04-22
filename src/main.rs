use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;
use std::process;

fn main() {
    println!("Hello, world!");
    let log = get_args().unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    // TODO: change to unwrap_or_else
    let file_data_iter = read_file(&log).unwrap();

    for line in file_data_iter {
        println!("{}", line.unwrap());
    }

    dbg!(&log);
}

fn get_args() -> std::result::Result<String, &'static str> {
    let mut args = env::args();
    args.next();

    let log_name = match args.next() {
        Some(arg) => arg,
        None => return Err("didn`t get log file."),
    };

    Ok(log_name)
}

fn read_file(file_name: &str) -> std::result::Result<Lines<BufReader<File>>, io::Error> {
    let input = File::open(file_name)?;
    //for line in BufReader::new(input).lines() {
    //    println!("{}", line?)
    //}

    Ok(BufReader::new(input).lines())
}

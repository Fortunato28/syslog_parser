use recap::{from_captures, Regex};
use serde::Deserialize;
use std::env;
use std::error::Error;
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

    //for line in file_data_iter {
    //    println!("{}", line.unwrap());
    //}
    //

    check_regex();

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

#[derive(Debug, Deserialize, PartialEq)]
struct LogMessage {
    facility: String,
    severity: String,
    timestamp: String,
    source_name: String,
    data: String,
}

//TODO: maybe do I need something more meaningful?
impl LogMessage {
    pub fn new() -> LogMessage {
        LogMessage {
            facility: String::new(),
            severity: String::new(),
            timestamp: String::new(),
            source_name: String::new(),
            data: String::new(),
        }
    }
}

fn check_regex() {
    let reg = Regex::new(r"<(\d{2})(\d{1})>(Nov\s\d+\s\d{2}:\d{2}:\d{2})\s(.+?):\s(.*)").unwrap();

    let log_string = "<166>Nov 13 15:38:01 10.181.233.206 %ASA-6-303002: FTP connection from outside:172.21.173.130/49845 to vos2:10.81.123.19/21, user comersant Stored file 2014-11-13_15-33-29---1---megafon-out.wav";

    for cap in reg.captures_iter(log_string) {
        println!(
            "1: {}\n 2: {}\n 3: {}\n 4: {}\n 5: {}\n",
            &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_recap() {
        let pattern = Regex::new(r#"(?P<facility>\S+)\s(?P<severity>\S+)\s(?P<timestamp>\S+)\s(?P<source_name>\S+)\s(?P<data>\S+)"#).unwrap();

        let example: LogMessage = from_captures(&pattern, "hello there who is here").unwrap();

        assert_eq!(
            example,
            LogMessage {
                facility: "hello".into(),
                severity: "there".into(),
                timestamp: "who".into(),
                source_name: "is".into(),
                data: "here".into(),
            }
        );
    }

    #[test]
    fn check_regex() {
        let reg =
            Regex::new(r"<(\d{2})(\d{1})>(Nov\s\d+\s\d{2}:\d{2}:\d{2})\s(.+?):\s(.*)").unwrap();

        let log_string = "<166>Nov 13 15:38:01 10.181.233.206 %ASA-6-303002: FTP connection from outside:172.21.173.130/49845 to vos2:10.81.123.19/21, user comersant Stored file 2014-11-13_15-33-29---1---megafon-out.wav";

        for cap in reg.captures_iter(log_string) {
            println!(
                "1: {}\n 2: {}\n 3: {}\n 4: {}\n 5: {}\n",
                &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]
            );
        }
    }
}

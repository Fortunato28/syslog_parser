use recap::{from_captures, Regex};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::process;
mod get_different_sources;
use get_different_sources as cases;

fn main() {
    let mut vec_with_parsed_lines = Vec::new();
    vec_with_parsed_lines.reserve(7000);

    let log = get_args().unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    // Filling vector parsed lines
    for line in read_file(&log).unwrap() {
        vec_with_parsed_lines.push(ParsedLine::new(&line.unwrap()));
    }

    cases::perform_cases(&vec_with_parsed_lines);
}

fn get_args() -> Result<String, &'static str> {
    let mut args = env::args();
    args.next();

    let log_name = match args.next() {
        Some(arg) => arg,
        None => return Err("didn`t get log file."),
    };

    Ok(log_name)
}

fn read_file(file_name: &str) -> Result<Lines<BufReader<File>>, io::Error> {
    let input = File::open(file_name)?;

    Ok(BufReader::new(input).lines())
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ParsedLine {
    pub facility: String,
    pub severity: String,
    pub timestamp: String,
    pub hostname: String,
    pub appname: String,
    pub data: String,
}

impl ParsedLine {
    pub fn new(line: &str) -> ParsedLine {
        let pattern = Regex::new(
            r"(?x)
        <
        (?P<facility>\d{2})
        (?P<severity>\d{1})
        >
        (?P<timestamp>\w{3}\s\d{2}\s\d{2}:\d{2}:\d{2})\s
        (?P<hostname>.+?)
        \s
        (?P<appname>.+?)
        :\s
        (?P<data>.*)",
        )
        .unwrap();

        // TODO: ugly double regex, zip it
        let parsed_line: ParsedLine = from_captures(&pattern, line).unwrap_or_else(|err| {
            eprintln!("Some bad line: {}", err);

            let bad_pattern = Regex::new(
                r"(?x)
        <
        (?P<facility>\d{2})
        (?P<severity>\d{1})
        >
        (?P<timestamp>\w{3}\s\d{2}\s\d{2}:\d{2}:\d{2})\s
        (?P<hostname>\S+?)
        \s
        (?P<appname>.+?)
        \s
        (?P<data>.*)",
            )
            .unwrap();

            let bad_parsed_line: ParsedLine = from_captures(&bad_pattern, line).unwrap();
            bad_parsed_line
        });
        parsed_line
    }
}

//TODO: fix test, it must be normal unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_recap() {
        let pattern = Regex::new(
            r"(?x)
            <
            (?P<facility>\d{2})
            (?P<severity>\d{1})
            >
            (?P<timestamp>\w{3}\s\d{2}\s\d{2}:\d{2}:\d{2})\s
            (?P<hostname>.*)
            \s
            (?P<appname>.+?)
            :\s
            (?P<data>.*)",
        )
        .unwrap();

        let log_string = "<166>Nov 13 15:38:01 10.181.233.206 %ASA-6-303002: FTP connection";
        let example: ParsedLine = from_captures(&pattern, log_string).unwrap();

        assert_eq!(
            example,
            ParsedLine {
                facility: "16".into(),
                severity: "6".into(),
                timestamp: "Nov 13 15:38:01".into(),
                hostname: "10.181.233.206".into(),
                appname: "%ASA-6-303002".into(),
                data: "FTP connection".into(),
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

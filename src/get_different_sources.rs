use crate::ParsedLine;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

// TODO: Применить все кейсы за один проход по логу
pub fn perform_cases(parsed_data: &Vec<ParsedLine>) {
    let mut sources = Vec::new();
    let mut asa_messages: Vec<String> = Vec::new();
    let mut asa_ips: Vec<String> = Vec::new();

    parsed_data.iter().for_each(|line| {
        // Case 1
        if !sources.contains(&line.hostname) {
            sources.push(line.hostname.clone())
        }

        // Cases 2-3
        if line.appname.contains("ASA") {
            // Case 2
            if !asa_messages.contains(&line.appname[7..13].into()) {
                asa_messages.push(line.appname.clone()[7..13].to_string());
            }

            // Case 3
            let reg = Regex::new(r"(\d+\.\d+\.\d+\.\d+).*?(\d+\.\d+\.\d+\.\d+)").unwrap();
            let ip = &line.data;
            if reg.is_match(&ip) {
                let caps = reg.captures(&ip).unwrap();
                caps.iter().skip(1).for_each(|cap| {
                    //println!("{}", cap.unwrap().as_str());
                    if !asa_ips.contains(&cap.unwrap().as_str().into()) {
                        asa_ips.push(cap.unwrap().as_str().into());
                    }
                })
            }
        }
    });

    write_in_file("sources.txt", &sources).unwrap();
    write_in_file("asa_messages_type.txt", &asa_messages).unwrap();
    write_in_file("ips.txt", &asa_ips).unwrap();
}

fn write_in_file(file_name: &str, sources: &Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    for i in sources {
        write!(file, "{}\n", i)?;
    }
    Ok(())
}

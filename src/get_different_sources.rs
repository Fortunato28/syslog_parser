use crate::ParsedLine;
use std::fs::File;
use std::io::prelude::*;

// TODO: Применить все кейсы за один проход по логу
pub fn extract_diff_sources(parsed_data: &Vec<ParsedLine>) {
    let mut sources = Vec::new();
    let mut asa_messages: Vec<String> = Vec::new();

    parsed_data.iter().for_each(|line| {
        if !sources.contains(&line.hostname) {
            sources.push(line.hostname.clone())
        }

        if line.appname.contains("ASA") {
            println!("{}", &line.appname[7..13]);
            if !asa_messages.contains(&line.appname[7..13].into()) {
                asa_messages.push(line.appname.clone()[7..13].to_string());
            }
        }
    });

    write_in_file("sources.txt", &sources).unwrap();
    write_in_file("asa_messages_type.txt", &asa_messages).unwrap();
}

fn write_in_file(file_name: &str, sources: &Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    for i in sources {
        write!(file, "{}\n", i)?;
    }
    Ok(())
}

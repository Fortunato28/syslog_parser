use crate::ParsedLine;
use std::fs::File;
use std::io::prelude::*;

pub fn extract_diff_sources(parsed_data: &Vec<ParsedLine>) {
    let mut sources = Vec::new();

    parsed_data.iter().for_each(|line| {
        if !sources.contains(&line.source_name) {
            sources.push(line.source_name.clone())
        }
    });

    write_in_file(&sources).unwrap();
}

fn write_in_file(sources: &Vec<String>) -> std::io::Result<()> {
    let mut file = File::create("sources.txt")?;
    for i in sources {
        write!(file, "{}\n", i)?;
    }
    Ok(())
}

use std::fs;
use std::io::{BufRead, BufReader};

pub fn read_input(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line);
    }
    Ok(lines)
}

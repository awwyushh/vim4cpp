use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn load_file(path: &str) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().map(String::from).collect())
}

pub fn save_file(path: &str, content: &[String]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    for line in content {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

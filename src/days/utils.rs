use std::{fs::File, io::BufReader};

pub(crate) fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    use std::io::BufRead;
    let file = File::open(filename)?;
    BufReader::new(file).lines().collect()
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub type Error = Box<dyn std::error::Error>;

pub fn read_input<P: AsRef<Path>>(p: P) -> Result<Vec<String>, Error> {
    let file = File::open(p)?;
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line?)
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn get_name(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut names = Vec::new();

    for line in reader.lines() {
        names.push(line?);
    }

    Ok(names)
}

fn generate_random_numbers() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..10).map(|_| rng.gen_range(1..=10)).collect()
}

fn main() -> io::Result<()> {
    let names = get_name("config")?;

    for name in names {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&name)?;

        let random_numbers = generate_random_numbers();

        for num in &random_numbers {
            write!(file, "{} ", num)?;
        }
        writeln!(file)?;
    }

    Ok(())
}

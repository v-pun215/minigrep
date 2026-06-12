use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn grep(path: &String, target: &String) -> io::Result<()> {
    //print!("CALLED {} {}", path, target);
    if !Path::new(path).exists() {
        println!("{}: No such file or directory", path);
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }
    let file = File::open(path)?;
    let reader= BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(target) {
            println!("{}",line);
        }
    }

    Ok(())
}
fn main() {
    // mini grep
    let args: Vec<String> = env::args().collect();
    let string_search = &args[1];
    let path = &args[2];

    let _ = grep(&path, &string_search);
}

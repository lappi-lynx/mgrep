use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path: &String = &args[2];

    println!("Scanning for: {}", query);
    println!("File path: {}", path);

    let file_content = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    for line in search(query, &file_content) {
        println!("{line}");
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

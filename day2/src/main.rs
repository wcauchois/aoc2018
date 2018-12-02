use std::io;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn histogram(s: &str) -> HashMap<char, i32> {
    let mut h = HashMap::new();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }
    h
}

fn has_value(h: &HashMap<char, i32>, target: i32) -> bool {
    h.values().any(|&x| x == target)
}

fn get_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    reader.lines().collect::<io::Result<Vec<String>>>()
}

fn main() {
    let mut num_twos = 0;
    let mut num_threes = 0;
    let lines = get_lines("input.txt").unwrap();
    for line in lines {
        let hist = histogram(&line);
        if has_value(&hist, 2) {
            num_twos += 1;
        }
        if has_value(&hist, 3) {
            num_threes += 1;
        }
    }
    println!("{}", num_twos * num_threes);
}

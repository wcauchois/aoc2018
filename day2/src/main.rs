#![allow(dead_code)]

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

fn part_one() {
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

// If they differ by one char, returns the index of that char.
fn differs_by_one_char(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        let differing_chars: Vec<(usize, (char, char))> = s1.chars().zip(s2.chars()).enumerate().filter(|(_, (c1, c2))| c1 != c2).collect();
        match differing_chars.as_slice() {
            [(i, (_, _))] => Some(*i),
            _ => None
        }
    }
}

fn exclude_index(s: &str, i: usize) -> String {
    let (first, _) = s.split_at(i);
    let (_, second) = s.split_at(i + 1);
    format!("{}{}", first, second)
}

fn part_two() {
    let mut candidates = get_lines("input.txt").unwrap();
    while !candidates.is_empty() {
        let (first, elements) = candidates.split_first().unwrap();
        for elem in elements {
            if let Some(i) = differs_by_one_char(first, elem) {
                println!("{}", exclude_index(first, i));
                return
            }
        }
        candidates.remove(0);
    }
}

fn main() {
    part_two()
}

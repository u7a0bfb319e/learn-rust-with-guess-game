use colored::*;
use regex::Regex;
use std::io;

fn reponse_invalid() -> (u32, u32) {
    let msg: String = "Please input the range of the number or press enter play with default 0-3"
        .bright_green()
        .to_string();
    println!("{} {}", "Invalid input.".red(), msg);
    return get_range();
}

pub fn get_range() -> (u32, u32) {
    let range = read_input();
    let cap = parse_range(&range);

    let start: u32 = match cap.0.parse() {
        Ok(x) => x,
        Err(_) => return reponse_invalid(),
    };
    let end: u32 = match cap.1.parse() {
        Ok(x) => x,
        Err(_) => return reponse_invalid(),
    };

    if start == end || start > end {
        return reponse_invalid();
    }

    (start, end)
}

pub fn read_input() -> String {
    let mut range: String = String::new();
    io::stdin()
        .read_line(&mut range)
        .expect("Failed to read line");
    range
}

fn parse_range(range: &String) -> (&str, &str) {
    let re: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    match re.captures(&range) {
        Some(c) => return (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()),
        None => {
            if range.trim() == "" {
                return ("0", "3");
            } else {
                return ("0", "0");
            }
        }
    };
}

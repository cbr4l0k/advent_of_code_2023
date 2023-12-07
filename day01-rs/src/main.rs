use clap::{command, arg};
use std::fs;
use std::collections::HashMap;

fn main() {

    let m = command!("Advent of Code 2023 <ADD-DAY>") 
        .author("cbr4l0k")
        .propagate_version(true)
        .arg_required_else_help(true)
        .arg(arg!(-p --part <PART> "Possible values: 1 or 2")
             .required(true)
             .value_parser(["1", "2"])
             .help("Part of the challenge to run")
            )
        .arg(arg!(-f --filepath <FILEPATH> "String to path")
             .default_value("./input.txt")
             .help("Path to the file")
            ).get_matches();

    if m.contains_id("part") && m.contains_id("filepath") {

        let path = m.get_one::<String>("filepath").unwrap();
        let data = fs::read_to_string(path).expect("Unable to read file");

        if let Some(part) = m.get_one::<String>("part") {
            match part.as_str() {
                "1" => {
                    let mut value = 0;
                    data.lines()
                        .for_each(|line| {
                            value += find_and_add(&line).unwrap();
                        });
                    println!("solution: {}", value);
                },
                "2" => {
                    let mut value = 0;
                    data.lines()
                        .for_each(|line| {
                            value += find_and_add(find_and_replace(&line).as_str()).unwrap();
                        });
                    println!("solution: {}", value);
                },
                _ => {unreachable!();} 
            }
        }
    }
}

fn find_and_add(line: &str) -> Option<u32> {
    let a: Option<u32> = line.chars().find(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap());
    let b: Option<u32> = line.chars().rev().find(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap());

    match (a,b) {
        (Some(a), Some(b)) => {
            format!("{}{}",a,b).parse().ok()
        },
        _ => None,
    }
}

fn find_and_replace(line: &str) -> String{

    let mut hashtable: HashMap<&str, &str> = HashMap::new();
    hashtable.extend(vec![
                     ("one", "one1one"),
                     ("two", "two2two"),
                     ("three", "three3three"),
                     ("four", "four4four"),
                     ("five", "five5five"),
                     ("six", "six6six"),
                     ("seven", "seven7seven"),
                     ("eight", "eight8eight"),
                     ("nine", "nine9nine"),
    ]);

    hashtable.iter().fold(line.to_string(), | acc, (key, value)| {
        acc.replace(key, value)
    })
}

use std::{
    env,
    fs::{self},
    vec,
};
type SourceInput = Vec<u64>;
#[derive(Debug)]
struct CategoryMap {
    id: usize,
    dst: u64,
    src: u64,
    length: u64,
}

fn main() {
    let input_path = env::var("aoc_2023_05_path").unwrap() + "/input.txt";
    let input = fs::read_to_string(&input_path).unwrap();

    let (mut src, categories_map) = parse_input(input);
    let mut src_copy = src.clone();
    let result = calc_part1(&mut src_copy, categories_map);
    println!("{:?}", result);
}
fn calc_part1(mut src: &mut SourceInput, categories_map: Vec<CategoryMap>) -> u64 {
    let mut cat_id = 1;
    let mut found: Vec<u64> = vec![];
    let mut not_found: Vec<u64> = vec![];

    while cat_id < 8 {
        while src.len() > 0 {
            let s = src.pop().unwrap();
            not_found.push(s);
            for c in &categories_map {
                if c.id == cat_id {
                    if s >= c.src && s < c.src + c.length {
                        let a = (s + c.dst) - c.src;
                        found.push(a);
                        not_found.pop();
                    }
                } else {
                }
            }
        }
        found.append(&mut not_found);
        *src = found;
        not_found.clear();
        found = vec![];
        cat_id += 1;
    }

    let result = src.into_iter().min().unwrap();
    return *result;
}
fn parse_input(input: String) -> (SourceInput, Vec<CategoryMap>) {
    let mut src: Vec<u64> = vec![];
    let mut categories_map: Vec<CategoryMap> = vec![];
    let mut cat_id = 0;

    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if i == 0 {
            let mut word = String::new();
            for (j, c) in line.chars().skip(7).enumerate() {
                if c.is_ascii_digit() {
                    word.push(c);
                    if j == line.len() - 8 && !word.is_empty() {
                        src.push(u64::from_str_radix(&word, 10).unwrap());
                        word.clear();
                    }
                } else if (c == ' ' || j == line.len() - 1) && !word.is_empty() {
                    src.push(u64::from_str_radix(&word, 10).unwrap());
                    word.clear();
                }
            }
            continue;
        }
        if line.contains("map") {
            cat_id += 1;
            continue;
        }
        let mut word = String::new();
        let mut cm_counter = 0;
        let mut cm = CategoryMap {
            id: cat_id,
            dst: 0,
            src: 0,
            length: 0,
        };
        for (j, c) in line.char_indices() {
            if c.is_ascii_digit() {
                word.push(c);
                if j == line.len() - 1 {
                    cm.length = u64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                }
            } else if c == ' ' {
                if cm_counter == 0 {
                    cm_counter += 1;
                    cm.id = cat_id;
                    cm.dst = u64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                } else if cm_counter == 1 {
                    cm_counter += 1;

                    cm.src = u64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                } else if cm_counter == 2 {
                    cm_counter += 1;

                    cm.length = u64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                }
            }
        }
        categories_map.push(cm);
    }
    return (src, categories_map);
}

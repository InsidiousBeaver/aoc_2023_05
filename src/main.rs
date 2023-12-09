use std::{
    env,
    fs::{self},
    vec,
};
type SourceRanges = Vec<(i64, i64)>;
type SourceInput = Vec<i64>;
#[derive(Debug)]
struct CategoryMap {
    id: usize,
    dst: i64,
    src: i64,
    length: i64,
}

fn main() {
    let input_path = env::var("aoc_2023_05_path").unwrap() + "/input.txt";
    let input = fs::read_to_string(&input_path).unwrap();

    let (src, categories_map) = parse_input(input);

    let sr = convert_src_to_ranges(&src);
    let min = calc_part2(sr, categories_map);
    println!("min: {}", min);
}
fn calc_part2(mut src: SourceRanges, categories_map: Vec<CategoryMap>) -> i64 {
    let mut temp_src: SourceRanges = vec![];

    for cat_id in 1..8 {
        'sourceloop: for rng in &mut src {
            for cat in &categories_map {
                if cat.id == cat_id {
                    if rng.0 <= cat.src + cat.length - 1 && cat.src <= rng.0 + rng.1 - 1 {
                        let dst_src_dif = cat.dst - cat.src;
                        let a1 = i64::max(rng.0, cat.src);
                        let a2 = i64::min(rng.0 + rng.1 - 1, cat.src + cat.length - 1);
                        let a11 = a1 + dst_src_dif;
                        let a22 = a2 - a1 + 1;
                        temp_src.push((a11, a22));
                        rng.0 = rng.0 + a22 - 1;
                        rng.1 = rng.1 - a22;
                    }
                    if rng.0 == 0 && rng.1 == 0 {
                        continue 'sourceloop;
                    }
                }
            }
            if rng.1 > 0 {
                temp_src.push(*rng);
                rng.0 = 0;
                rng.1 = 0;
            }
        }

        src = temp_src;
        println!("{:?}", src);
        temp_src = vec![];
    }
    let mut min = i64::MAX;
    for s in src {
        if s.0 < min && s.1 > 0 {
            min = s.0;
        }
    }
    return min;
}
fn convert_src_to_ranges(src: &SourceInput) -> SourceRanges {
    let mut new_src = vec![];
    let mut pair: Option<(i64, i64)> = None;
    for n in src {
        match pair {
            Some(mut p) => {
                p.1 = *n;
                new_src.push(p);
                pair = None;
            }
            None => pair = Some((*n, 0)),
        }
    }
    return new_src;
}
fn calc_part1(src: &mut SourceInput, categories_map: Vec<CategoryMap>) -> i64 {
    let mut cat_id = 1;
    let mut found: Vec<i64> = vec![];
    let mut not_found: Vec<i64> = vec![];

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
    let mut src: Vec<i64> = vec![];
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
                        src.push(i64::from_str_radix(&word, 10).unwrap());
                        word.clear();
                    }
                } else if (c == ' ' || j == line.len() - 1) && !word.is_empty() {
                    src.push(i64::from_str_radix(&word, 10).unwrap());
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
                    cm.length = i64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                }
            } else if c == ' ' {
                if cm_counter == 0 {
                    cm_counter += 1;
                    cm.id = cat_id;
                    cm.dst = i64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                } else if cm_counter == 1 {
                    cm_counter += 1;

                    cm.src = i64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                } else if cm_counter == 2 {
                    cm_counter += 1;

                    cm.length = i64::from_str_radix(&word, 10).unwrap();
                    word.clear();
                }
            }
        }
        categories_map.push(cm);
    }
    return (src, categories_map);
}

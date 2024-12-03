use regex::Regex;
use std::fs;

fn part1(matches: Vec<String>) -> u32 {
    matches
        .iter()
        .filter_map(|mul| {
            let (x, y) = mul
                .strip_prefix("mul(")?
                .strip_suffix(")")?
                .split_once(",")?;

            Some(x.parse::<u32>().ok()? * y.parse::<u32>().ok()?)
        })
        .sum()
}

fn part2(matches: Vec<String>) -> u32 {
    let mut ms = matches.clone();
    let mut draining = false;

    for i in 0..ms.len() {
        match ms[i].as_str() {
            "do()" => draining = false,
            "don't()" => draining = true,
            _ => {}
        }
        if draining {
            ms[i] = String::from("mul(0,0)");
        }
    }

    ms = ms
        .into_iter()
        .filter(|s| s.as_str() != "do()")
        .collect::<Vec<String>>();

    part1(ms)
}

pub fn main() {
    let file_content: String = fs::read_to_string("input.txt").unwrap().replace("\n", "");

    // let re_part1: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let re_part2: Regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let matches: Vec<String> = re_part2
        .find_iter(&file_content)
        .map(|m| String::from(m.as_str()))
        .collect();

    let res: u32 = part2(matches);
    println!("{:?}", res);
}

use std::collections::HashMap;
use std::fs;

const PATH: &str = "./input.txt";

fn input_to_lists(s: String) -> (Vec<u32>, Vec<u32>) {
    s.lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok());
            match (nums.next(), nums.next()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            }
        })
        .unzip()
}

fn part_1(list_a: Vec<u32>, list_b: Vec<u32>) -> u32 {
    list_a
        .iter()
        .zip(list_b)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

fn part_2(list_a: Vec<u32>, list_b: Vec<u32>) -> u32 {
    let mut m: HashMap<u32, u32> = HashMap::new();

    for n in list_a {
        m.insert(n, 0);
    }

    for n in list_b {
        let val = m.get(&n);
        match val {
            Some(_) => m.insert(n, val.unwrap() + 1),
            None => continue,
        };
        // *m.entry(n).or_default() += 1
    }

    m.iter().map(|(key, val)| key * val).sum()
}

fn main() {
    let file_content = fs::read_to_string(PATH).expect("error reading file");

    let (mut list_a, mut list_b): (Vec<u32>, Vec<u32>) = input_to_lists(file_content);

    list_a.sort();
    list_b.sort();

    // let result: u32 = part_1(list_a, list_b);
    let result = part_2(list_a, list_b);
    println!("{}", result);
}

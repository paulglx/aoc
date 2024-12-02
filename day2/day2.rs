use std::fs;

const PATH: &str = "./input.txt";

fn is_sorted(v: Vec<u32>) -> bool {
    v.is_sorted_by(|a, b| a <= b) || v.is_sorted_by(|a, b| a >= b)
}

fn valid_gaps(v: Vec<u32>) -> bool {
    v.windows(2)
        .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
}

fn is_valid(v: Vec<u32>) -> bool {
    is_sorted(v.clone()) && valid_gaps(v.clone())
}

fn is_valid_dampened(v: Vec<u32>) -> bool {
    for i in 0..v.len() {
        let mut vd = v.clone();
        vd.remove(i);

        if is_valid(vd) {
            return true;
        };
    }
    false
}

pub fn main() {
    let file_content = fs::read_to_string(PATH).expect("err");

    let reports: Vec<Vec<u32>> = file_content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| -> u32 { s.parse::<u32>().unwrap() })
                .collect()
        })
        .collect();

    let valid_reports: Vec<Vec<u32>> = reports
        .into_iter()
        .filter(|r| is_valid(r.clone()) || is_valid_dampened(r.clone()))
        .collect::<Vec<Vec<u32>>>();

    println!("{:?}", valid_reports);
    println!("{:?}", valid_reports.len());
}

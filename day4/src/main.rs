use fancy_regex::Regex;
use std::fs;

// Part 1
fn transpose(s: &str) -> String {
    // Tranposes a string like it's a matrix, with lines as rows

    let mat: Vec<String> = s.lines().map(|l| l.to_owned()).collect::<Vec<String>>();

    let mat_as_vec_of_chars = mat
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = mat_as_vec_of_chars.len();
    let cols = mat_as_vec_of_chars[0].len();

    let transposed = (0..cols)
        .map(|col| (0..rows).map(|row| mat_as_vec_of_chars[row][col]).collect())
        .collect::<Vec<Vec<char>>>();

    transposed
        .into_iter()
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

// Part 1
fn check_diagonals(s: &str) -> usize {
    // string into Vec<Vec<char>>
    let mat = s
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // let mut square: Vec<Vec<char>>;
    let mut diagonal_1: Vec<char>;
    let mut diagonal_2: Vec<char>;

    let mut result: usize = 0;

    let rows = mat.len();
    let cols = mat[0].len();

    for i in 0..rows - 3 {
        for j in 0..cols - 3 {
            let square = mat[i..i + 4]
                .iter()
                .map(|line| line[j..j + 4].to_vec())
                .collect::<Vec<Vec<char>>>();
            //
            // let display = square
            //     .iter()
            //     .map(|line| line.iter().collect::<String>())
            //     .collect::<Vec<String>>()
            //     .join("\n");

            // println!("Square : \n{}\n", display);

            diagonal_1 = (0..4).map(|j| square[j][j]).collect();
            diagonal_2 = (0..4).map(|j| square[j][3 - j]).collect();

            if diagonal_1 == ['X', 'M', 'A', 'S'] || diagonal_1 == ['S', 'A', 'M', 'X'] {
                result += 1;
                // println!("Found here! res={}", result);
            }
            if diagonal_2 == ['X', 'M', 'A', 'S'] || diagonal_2 == ['S', 'A', 'M', 'X'] {
                result += 1;
                // println!("Found here! res={}", result);
            }
        }
    }

    result
}

fn main() {
    let file_content = fs::read_to_string("./input.txt").unwrap();

    let xmas_regex: Regex = Regex::new(r"(?=(XMAS|SAMX))").unwrap();
    //
    // // horizontal lines
    let horizontal_matches = xmas_regex.find_iter(&file_content).count();
    // // vertical lines : transpose and re-check
    let transposed_content = transpose(&file_content);
    let vertical_matches = xmas_regex.find_iter(&transposed_content).count();
    // // check_diagonals
    let diags: usize = check_diagonals(&file_content);
    //
    // let final_result = horizontal_matches + vertical_matches + diags;

    let final_result = part2(&file_content);

    println!("{}", final_result);
}

// Part 2
fn part2(s: &str) -> u32 {
    // Super naive solution bc it's late :)

    let mat = s
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = mat.len();
    let cols = mat[0].len();

    let mut result: u32 = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if mat[i][j] == 'A' {
                let diag_1: String =
                    format!("{}{}{}", mat[i - 1][j - 1], mat[i][j], mat[i + 1][j + 1]);
                let diag_2: String =
                    format!("{}{}{}", mat[i + 1][j - 1], mat[i][j], mat[i - 1][j + 1]);
                if (&diag_1 == "SAM" || &diag_1 == "MAS") && (&diag_2 == "SAM" || &diag_2 == "MAS")
                {
                    result += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_transposes() {
        let input = "ABC
DEF
GHI";
        let transposed = transpose(input);
        assert_eq!(
            transposed,
            "ADG
BEH
CFI"
        );
    }

    #[test]
    fn it_counts_diagonal_xmases() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = check_diagonals(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn it_counts_hor_and_vert() {
        // string contains 5 horizontal XMAS|SAMX and 3 vertical ones
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let xmas_regex: Regex = Regex::new(r"(?=(XMAS|SAMX))").unwrap();

        // horizontal lines
        let horizontal_matches = xmas_regex.find_iter(input).count();

        // vertical lines : transpose and re-check
        let transposed_content = transpose(input);
        let vertical_matches = xmas_regex.find_iter(&transposed_content).count();

        assert_eq!(horizontal_matches + vertical_matches, 8);
    }

    #[test]
    fn it_counts_x_mas_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = part2(input);

        assert_eq!(result, 9);
    }
}

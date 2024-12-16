use std::fs;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

struct Pos {
    x: usize,
    y: usize,
}

fn print_grid(grid: &[Vec<char>]) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

// Gets vector from Direction
fn get_direction(direction: &mut Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    }
}

// Sets char at position in grid
fn grid_set(grid: &mut [Vec<char>], position: &mut Pos, c: char) {
    grid[position.y][position.x] = c;
}

fn rotate(direction: &mut Direction) {
    match direction {
        Direction::Up => *direction = Direction::Right,
        Direction::Right => *direction = Direction::Down,
        Direction::Down => *direction = Direction::Left,
        Direction::Left => *direction = Direction::Up,
    };
}

// Walks in direction and writes previous location on map
// Assumes we can walk
fn walk(grid: &mut [Vec<char>], position: &mut Pos, direction: &mut Direction) -> bool {
    grid_set(grid, position, 'X');
    let (dir_x, dir_y) = get_direction(direction);

    let next_x = (position.x as i32) + dir_x;
    let next_y = (position.y as i32) + dir_y;

    // println!("next : ({}, {})", next_x, next_y);

    if next_x < 0 || next_x >= grid[0].len() as i32 || next_y < 0 || next_y >= grid.len() as i32 {
        println!("Stopping : out of bounds");
        return false;
    }

    if grid[next_y as usize][next_x as usize] == '#' {
        // print_grid(grid);
        rotate(direction);
        return true;
    }

    position.x = next_x as usize;
    position.y = next_y as usize;

    true
}

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut direction: Direction = Direction::Up;
    let mut position: Pos = Pos { x: 0, y: 0 };

    // Fill grid
    let file_content = fs::read_to_string("input.txt").unwrap();
    for line in file_content.lines() {
        grid.push(line.chars().collect());
    }

    let height = grid.len();
    let width = grid[0].len();

    // find initial direction
    for j in 0..height {
        for i in 0..width {
            if grid[j][i] == '^' {
                position.x = i;
                position.y = j;
                break;
            }
        }
    }

    println!("({}, {})", position.x, position.y);

    let mut cont: bool = true;

    while cont {
        cont = walk(&mut grid, &mut position, &mut direction);
    }
    print_grid(&grid);

    println!("Done walking");

    // Count Xes
    let result = grid.into_iter().flatten().filter(|c| *c == 'X').count();
    println!("Total number of X : {}", result);
}

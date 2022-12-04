use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("input.txt").expect("Failed to open input");

    let mut score: u64 = 0;
    for line_result in lines {
        let line_str = line_result.unwrap();
        let line = line_str.as_bytes();
        let opponent_shape = line[0] - 'A' as u8;
        let my_shape = line[2] - 'X' as u8;
        let state_score: u64 = match (opponent_shape - my_shape) as i8 {
            -2 | 1 => 0,
            -1 | 2 => 6,
            0 => 3,
            _ => { println!("Error"); 0 }
        };
        score += state_score + my_shape as u64 + 1;
    }

    println!("Part 1: {}", score);
}

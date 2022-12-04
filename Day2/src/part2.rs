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
        let opponent_shape = (line[0] - 'A' as u8) as i8;
        let state_score = (line[2] - 'X' as u8) * 3;
        let shape_score = match state_score {
            0 => (opponent_shape - 1).rem_euclid(3),
            3 => opponent_shape,
            6 => (opponent_shape + 1).rem_euclid(3),
            _ => { println!("Got an error"); 0 }
        } as u8 + 1;
        score += (state_score + shape_score) as u64;
    }

    println!("Part 2: {}", score);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_priority(c: char) -> u64 {
    let b = c as u64;
    let a = 'a' as u64;
    let A = 'A' as u64;

    if b < a {
        b - A + 1
    } else {
        b - a + 27
    }
}

pub fn get_input() -> std::io::Lines<io::BufReader<File>> {
    read_lines("input.txt").expect("Failed to open input file")
}

pub fn calculate(lines: std::io::Lines<io::BufReader<File>>) -> u64 {
    let mut priorities_sum: u64 = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        let (first, second) = line.split_at(line.len() / 2);
        'search_loop: for char_first in first.chars() {
            for char_second in second.chars() {
                if char_first == char_second {
                    priorities_sum += get_priority(char_first);
                    break 'search_loop;
                }
            }
        }
    }

    priorities_sum
}

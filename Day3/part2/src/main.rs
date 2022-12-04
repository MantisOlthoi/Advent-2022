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
        b - A + 27
    } else {
        b - a + 1
    }
}

fn main() {
    let lines = read_lines("input.txt").expect("Failed to open input file");

    let mut priorities_sum: u64 = 0;
    let mut first_line = String::new();
    let mut second_line = String::new();
    let mut index = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        
        if index == 2 {
            'search_loop: for a in first_line.chars() {
                for b in second_line.chars() {
                    if a == b {
                        for c in line.chars() {
                            if c == a {
                                priorities_sum += get_priority(a);
                                break 'search_loop;
                            }
                        }
                    }
                }
            }
        }

        first_line = second_line;
        second_line = line;
        index = (index + 1) % 3;
    }
    assert!(index == 0);

    println!("{}", priorities_sum);
}

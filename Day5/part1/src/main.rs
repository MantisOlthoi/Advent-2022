use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn reverse_stack(in_stack: &mut Vec<char>) -> Vec<char> {
    let mut out_stack = vec![];
    while !in_stack.is_empty() {
        out_stack.push(in_stack.pop().unwrap());
    }
    out_stack
}

fn main() {
    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let lines = read_lines("input.txt").expect("Failed to read input file");

    let mut stacks = vec![];
    let mut num_stacks = 0;
    let mut is_first_line = true;
    'line_loop: for line_result in lines {
        let line = line_result.unwrap();

        if is_first_line {
            num_stacks = line.len() / 4 + 1;
            for _i in 0..num_stacks {
                stacks.push(Vec::<char>::new());
            }
            is_first_line = false;
        }

        for cap in move_regex.captures_iter(&line) {
            let move_count = cap[1].parse::<usize>().unwrap();
            let from_stack = cap[2].parse::<usize>().unwrap() - 1;
            let to_stack = cap[3].parse::<usize>().unwrap() - 1;

            for _i in 0..move_count {
                let c = stacks[from_stack].pop().unwrap();
                stacks[to_stack].push(c);
            }

            continue 'line_loop;
        }

        if line.len() > 0 {
            let line_chars = line.as_bytes();

            if (line_chars[1] as char).is_ascii_digit() {
                for i in 0..num_stacks {
                    stacks[i] = reverse_stack(&mut stacks[i]);
                }
                continue 'line_loop;
            }

            for i in (1..=(num_stacks*4)).step_by(4) {
                let stack_id = i / 4;
                let c = line_chars[i] as char;
                if c != ' ' {
                    stacks[stack_id].push(c);
                }
            }
        }
    }

    let mut result: String = String::new();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }

    println!("{}", result);
}

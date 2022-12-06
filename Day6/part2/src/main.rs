use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("input.txt").unwrap();

    for line_result in lines {
        let line = line_result.unwrap();

        const NUM_UNIQ: usize = 14;
        let mut position = 0;
        let mut read_chars: [char; NUM_UNIQ] = ['\0'; NUM_UNIQ];
        'char_loop: for c in line.chars() {
            if position >= NUM_UNIQ {
                let mut none_match = true;
                'match_loop: for i in 0..=(NUM_UNIQ - 2) {
                    for j in (i+1)..=(NUM_UNIQ-1) {
                        if read_chars[i] == read_chars[j] {
                            none_match = false;
                            break 'match_loop;
                        }
                    }
                }
                if none_match {
                    println!("{}", position);
                    break 'char_loop;
                }
            }

            read_chars[position % NUM_UNIQ] = c;
            position += 1;
        }
    }
}

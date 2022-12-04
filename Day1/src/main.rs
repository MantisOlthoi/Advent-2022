use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let input_filename = "input.1.txt";
    let input_lines = read_lines(input_filename).expect("Failed to read input file");

    let mut elves_calories = Vec::<u64>::new();
    let mut current_elf_calories: u64 = 0;
    let mut most_calories_index = 0;
    let mut most_calories: u64 = 0;
    let mut i = 0;
    for line_result in input_lines {
        let line = line_result.unwrap();
        if line.trim().len() == 0 {
            if most_calories == 0 || most_calories < current_elf_calories {
                most_calories_index = i;
                most_calories = current_elf_calories;
            }

            elves_calories.push(current_elf_calories);
            current_elf_calories = 0;
            i += 1;
            continue;
        }

        let line_calories = line.parse::<u64>().expect(&format!("Unable to parse {}", line));
        current_elf_calories += line_calories;
    }

    println!("Part1: {}", most_calories);

    elves_calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_three_sum = elves_calories[0] + elves_calories[1] + elves_calories[2];

    println!("Part2: {}", top_three_sum);
}

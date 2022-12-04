use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("input.txt").expect("Failed to find input file");

    let mut count: u64 = 0;
    for line_result in lines {
        let line = line_result.unwrap();

        let (first_elf,second_elf) = line.split_once(',').unwrap();
        let (first_elf_start_str, first_elf_end_str) = first_elf.split_once('-').unwrap();
        let (second_elf_start_str, second_elf_end_str) = second_elf.split_once('-').unwrap();

        let first_elf_start = first_elf_start_str.parse::<u16>().unwrap();
        let first_elf_end = first_elf_end_str.parse::<u16>().unwrap();
        let second_elf_start = second_elf_start_str.parse::<u16>().unwrap();
        let second_elf_end = second_elf_end_str.parse::<u16>().unwrap();

        if (first_elf_start <= second_elf_start && first_elf_end >= second_elf_end)
           || (second_elf_start <= first_elf_start && second_elf_end >= first_elf_end) {
            count += 1;
        }
    }

    println!("{}", count);
}

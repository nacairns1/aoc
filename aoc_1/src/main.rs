use std::error::Error;

use aoc_1::{Elf, sort_descending_in_total_cals};

// Complete!



fn main() -> Result<(), Box<dyn Error>> {
    let input: String = std::fs::read_to_string("input.txt")?;

    let mut elves = Elf::parse_string_into_elves(input);

    sort_descending_in_total_cals(&mut elves);

    let first_three_elves = elves.iter().take(3).fold(0,|cal_pool, elf|cal_pool+&elf.total_calories());

    println!("{}", first_three_elves);
    Ok(())
}

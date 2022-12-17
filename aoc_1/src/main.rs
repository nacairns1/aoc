use std::error::Error;

use aoc_1::*;




fn main() -> Result<(), Box<dyn Error>> {
    let input: String = std::fs::read_to_string("input.txt")?;

    let elves_as_str = input.split('\n').collect::<Vec<&str>>();    
    for elf_str in elves_as_str {

    };

    println!("{:?}", elves_as_str.get(3).unwrap());

    let elf_queue = ElfQueue::from(input.to_owned());

    println!("Highest Calorie Elf: {}", elf_queue.get(0).unwrap().total_calories());
    Ok(())
}

#[derive(PartialEq, Eq)]
pub struct Elf {
    snacks: Vec<Snack>,
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_tot_cals = self.total_calories();
        let oth_tot_cals = other.total_calories();

        self_tot_cals.cmp(&oth_tot_cals)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_calories().partial_cmp(&other.total_calories())
    }
}

pub fn sort_descending_in_total_cals(elves: &mut Vec<Elf>) {
    elves.sort_by(|elf_1, elf_2| elf_2.cmp(elf_1));
}

pub type Snack = u64;

impl Elf {
    pub fn parse_string_into_elves(s: String) -> Vec<Elf> {
        let mut elf_vec: Vec<Elf> = Vec::new();
        let lines = s.lines();

        let mut snack_queue: Vec<Snack> = Vec::new();

        for line in lines {
            if !line.is_empty() {
                snack_queue.push(line.parse::<Snack>().expect("Failed to parse snack"));
            } else {
                let snacks = snack_queue.clone();
                let new_elf = Elf { snacks };
                elf_vec.push(new_elf);
                snack_queue = Vec::new();
            }
        }

        elf_vec
    }

    pub fn total_calories(&self) -> u64 {
        self.snacks
            .iter()
            .fold(0, |snack_pool, snack| snack_pool + snack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = r"
10
20
30

40
50
60

10
20
60

70
80";

    #[test]
    fn four_elves_parsed_from_input() {
        let wanted_output = 4usize;
        let elves = Elf::parse_string_into_elves(input.to_owned());

        assert_eq!(elves.len(), wanted_output);
    }
}

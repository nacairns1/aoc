use rps::{EnemyChoice, compare_choices};

use crate::rps::Outcome;

pub mod rps;

fn main() {
    let input_string = std::fs::read_to_string("input.txt").unwrap();

    let tot_pts = input_string.lines().fold(0, |mut tot, line|  {
        let mut choices = line.split_whitespace();
        let enemy_choice_str = choices.next().unwrap();
        let outcome_str = choices.next().unwrap();

        
        let ec = EnemyChoice::from_str(enemy_choice_str);
        let o = Outcome::from_str(outcome_str);


        tot += compare_choices(ec, o);
        tot
    });

    println!("{}", tot_pts);
}

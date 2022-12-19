pub enum EnemyChoice {
    A,
    B,
    C,
}

impl EnemyChoice {
    pub fn from_str(s: &str) -> Self {
        match s {
            "A" => EnemyChoice::A,
            "B" => EnemyChoice::B,
            "C" => EnemyChoice::C,
            _ => panic!("Unimplemented choice found for EnemyChoice"),
        }
    }
}
impl Outcome {
    pub fn from_str(s: &str) -> Self {
        match s {
            "X" => Outcome::Defeat,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Victory,
            _ => panic!("Unimplemented choice found for PlayerOutcome"),
        }
    }

    pub fn determine_player_choice(&self, ec: &EnemyChoice) -> PlayerChoice {
        match self {
            Outcome::Victory => match ec {
                EnemyChoice::A => PlayerChoice::Y,
                EnemyChoice::B => PlayerChoice::Z,
                EnemyChoice::C => PlayerChoice::X,
            },
            Outcome::Draw => match ec {
                EnemyChoice::A => PlayerChoice::X,
                EnemyChoice::B => PlayerChoice::Y,
                EnemyChoice::C => PlayerChoice::Z,
            },
            Outcome::Defeat => match ec {
                EnemyChoice::A => PlayerChoice::Z,
                EnemyChoice::B => PlayerChoice::X,
                EnemyChoice::C => PlayerChoice::Y,
            },
        }

    }
}

pub enum PlayerChoice {
    X,
    Y,
    Z,
}

impl PlayerChoice {
    pub fn from_str(s: &str) -> Self {
        match s {
            "X" => PlayerChoice::X,
            "Y" => PlayerChoice::Y,
            "Z" => PlayerChoice::Z,
            _ => panic!("Unimplemented choice found for PlayerChoice"),
        }
    }
}

pub enum Outcome {
    Victory,
    Draw,
    Defeat,
}

pub fn earn_player_choice_points(pc: &PlayerChoice) -> u32 {
    match pc {
        PlayerChoice::X => 1,
        PlayerChoice::Y => 2,
        PlayerChoice::Z => 3,
    }
}

pub fn determine_victory(ec: &EnemyChoice, pc: &PlayerChoice) -> Outcome {
    match ec {
        EnemyChoice::A => match pc {
            PlayerChoice::X => Outcome::Draw,
            PlayerChoice::Y => Outcome::Victory,
            PlayerChoice::Z => Outcome::Defeat,
        },
        EnemyChoice::B => match pc {
            PlayerChoice::X => Outcome::Defeat,
            PlayerChoice::Y => Outcome::Draw,
            PlayerChoice::Z => Outcome::Victory,
        },
        EnemyChoice::C => match pc {
            PlayerChoice::X => Outcome::Victory,
            PlayerChoice::Y => Outcome::Defeat,
            PlayerChoice::Z => Outcome::Draw,
        },
    }
}

pub fn determine_outcome_pts(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Victory => 6,
        Outcome::Draw => 3,
        Outcome::Defeat => 0,
    }
}

pub fn compare_choices(ec: EnemyChoice, o: Outcome) -> u32 {
    let pc = o.determine_player_choice(&ec);

    let choice_pts = earn_player_choice_points(&pc);
    let outcome_pts = determine_outcome_pts(&o);

    choice_pts + outcome_pts
}

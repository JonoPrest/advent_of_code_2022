use anyhow::{anyhow, Context};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn try_from_string(string: String) -> anyhow::Result<Self> {
        match string.as_str() {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            val => Err(anyhow!("{} is not a valid move", val)),
        }
    }

    pub fn get_outcome_shape(&self, outcome: RoundOutcome) -> Self {
        match outcome {
            RoundOutcome::Lose => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            RoundOutcome::Draw => self.clone(),
            RoundOutcome::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
        }
    }

    pub fn to_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug)]
pub struct Round {
    opponent: Shape,
    player: Shape,
}

#[derive(Debug, Clone)]
pub enum RoundOutcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl RoundOutcome {
    pub fn try_from_string(string: String) -> anyhow::Result<RoundOutcome> {
        match string.as_str() {
            "X" => Ok(RoundOutcome::Lose),
            "Y" => Ok(RoundOutcome::Draw),
            "Z" => Ok(RoundOutcome::Win),
            val => Err(anyhow!("{} is not a valid round-outcome", val)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoundStrategy {
    opponent: Shape,
    player_outcome: RoundOutcome,
}

impl From<RoundStrategy> for Round {
    fn from(round_strat: RoundStrategy) -> Round {
        let RoundStrategy {
            opponent,
            player_outcome,
        } = round_strat;
        let player = opponent.get_outcome_shape(player_outcome);
        Round { opponent, player }
    }
}

impl Round {
    pub fn to_player_round_outcome(&self) -> RoundOutcome {
        match self {
            Round { player, opponent } if opponent == player => RoundOutcome::Draw,
            Round {
                player: Shape::Rock,
                opponent: Shape::Scissors,
            }
            | Round {
                player: Shape::Paper,
                opponent: Shape::Rock,
            }
            | Round {
                player: Shape::Scissors,
                opponent: Shape::Paper,
            } => RoundOutcome::Win,
            _ => RoundOutcome::Lose,
        }
    }

    pub fn to_player_score(&self) -> i32 {
        self.to_player_round_outcome() as i32 + self.player.to_score()
    }
}

type Rounds = Vec<Round>;

pub fn parse_input_first(file: &File) -> anyhow::Result<Rounds> {
    let reader = BufReader::new(file);
    let mut rounds: Rounds = Vec::new();
    for line in reader.lines() {
        let line = line.context("reading line to string")?;
        let content = line.trim();

        let round = {
            let chars: Vec<_> = content.chars().collect();
            let opponent = chars.get(0).map_or_else(
                || Err(anyhow!("No char 0")),
                |char| {
                    Ok(Shape::try_from_string(char.to_string())
                        .context(format!("deserializing char {}", char))?)
                },
            )?;
            let player = chars.get(2).map_or_else(
                || Err(anyhow!("No char 2")),
                |char| {
                    Ok(Shape::try_from_string(char.to_string())
                        .context(format!("parsing char {}", char))?)
                },
            )?;

            Round { player, opponent }
        };

        rounds.push(round);
    }

    Ok(rounds)
}

type RoundStrategies = Vec<RoundStrategy>;

pub fn parse_input_second(file: &File) -> anyhow::Result<RoundStrategies> {
    let reader = BufReader::new(file);
    let mut rounds: RoundStrategies = Vec::new();
    for line in reader.lines() {
        let line = line.context("reading line to string")?;
        let content = line.trim();

        let round = {
            let chars: Vec<_> = content.chars().collect();
            let opponent = chars.get(0).map_or_else(
                || Err(anyhow!("No char 0")),
                |char| {
                    Ok(Shape::try_from_string(char.to_string())
                        .context(format!("deserializing char {}", char))?)
                },
            )?;
            let player_outcome = chars.get(2).map_or_else(
                || Err(anyhow!("No char 2")),
                |char| {
                    Ok(RoundOutcome::try_from_string(char.to_string())
                        .context(format!("parsing char {}", char))?)
                },
            )?;

            RoundStrategy {
                player_outcome,
                opponent,
            }
        };

        rounds.push(round);
    }

    Ok(rounds)
}

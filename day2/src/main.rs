use aoc_helpers::{AocSession, Day};
use std::{convert::Infallible, str::FromStr};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock, 
    Paper, 
    Scissors 
}

#[derive(Debug)]
struct RockPaperScissorsCommand {
    opponent_move: RPS,
    our_move: RPS,
}

struct Day2(Vec<RockPaperScissorsCommand>);

fn score_round(opponent: RPS, us: RPS) -> i32 {
    match (opponent, us) {
        // we lose
        (RPS::Paper, RPS::Rock) => 1,
        (RPS::Scissors, RPS::Paper) => 2,
        (RPS::Rock, RPS::Scissors) => 3,

        // there's a tie
        (RPS::Rock, RPS::Rock) => 4,
        (RPS::Paper, RPS::Paper) => 5,
        (RPS::Scissors, RPS::Scissors) => 6,

        // we win
        (RPS::Scissors, RPS::Rock) => 7,
        (RPS::Rock, RPS::Paper) => 8,
        (RPS::Paper, RPS::Scissors) => 9,
    }
}

fn rps_to_outcome(r: RPS) -> OurOutcome {
    match r {
        RPS::Scissors => OurOutcome::Win,
        RPS::Paper => OurOutcome::Draw,
        RPS::Rock => OurOutcome::Lose,
    }
}

enum OurOutcome {
    Win,
    Draw,
    Lose
}

fn score_for_rps(val: RPS) -> i32 {
    match val {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3
    }
}

fn score_for_part2_strategy(opponent: RPS, us: RPS) -> i32 {
    // Rock = X, Paper = Y, Scissors = Z
    // Lose,     Draw,      Win
    // 0,        3,         6
    match (opponent, rps_to_outcome(us)) {
            // we have 9 solutions again, e.g. 3 for Rock - whether we Lose (X), Draw(Y), Win(Z)
            (RPS::Paper, OurOutcome::Win) => 6 + score_for_rps(RPS::Scissors),
            (RPS::Paper, OurOutcome::Draw) => 3 + score_for_rps(RPS::Paper),
            (RPS::Paper, OurOutcome::Lose) => score_for_rps(RPS::Rock),
            
            (RPS::Rock, OurOutcome::Win) => 6 + score_for_rps(RPS::Paper),
            (RPS::Rock, OurOutcome::Draw) => 3 + score_for_rps(RPS::Rock),
            (RPS::Rock, OurOutcome::Lose) => score_for_rps(RPS::Scissors),
            
            (RPS::Scissors, OurOutcome::Win) => 6 + score_for_rps(RPS::Rock),
            (RPS::Scissors, OurOutcome::Draw) => 3 + score_for_rps(RPS::Scissors),
            (RPS::Scissors, OurOutcome::Lose) => score_for_rps(RPS::Paper),

            _ => unreachable!()
    }
}

impl Day for Day2 {
    const DAY: u8 = 2;
    
    fn from_input(input: String) -> Self {
        let lines = input.trim().split('\n').map(|line| {
            RockPaperScissorsCommand::from_str(line).unwrap()
        }).collect();

        return Self(lines)
    }
    
    fn first_part(&mut self) -> String {
        let scored = self.0.iter().map(|cmd| score_round(cmd.opponent_move, cmd.our_move));
        scored.collect::<Vec<i32>>().iter().sum::<i32>().to_string()
    }
    
    fn second_part(&mut self) -> String {
        let scored = self.0.iter().map(|cmd| score_for_part2_strategy(cmd.opponent_move, cmd.our_move));
        scored.collect::<Vec<i32>>().iter().sum::<i32>().to_string()
    }
}

fn parse_our_move(s: &str) -> RPS {
    match s {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => unreachable!()
    }
}

impl FromStr for RockPaperScissorsCommand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect::<Vec<_>>();

        match (parts[0], parts[1]) {
            ("A", cmd) => {
                Ok(RockPaperScissorsCommand { opponent_move: RPS::Rock, our_move: parse_our_move(cmd) })
            },
            ("B", cmd) => {
                Ok(RockPaperScissorsCommand { opponent_move: RPS::Paper, our_move: parse_our_move(cmd) })
            },
            ("C", cmd) => {
                Ok(RockPaperScissorsCommand { opponent_move: RPS::Scissors, our_move: parse_our_move(cmd) })
            },
            _ => unreachable!()
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    AocSession::new(2022)?.day::<Day2>();
    Ok(())
}

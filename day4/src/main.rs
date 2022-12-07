use anyhow::Result;
use aoc_helpers::{AocSession, Day};
use itertools::Itertools;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Day4Command {
    pair1: (u8, u8),
    pair2: (u8, u8),
}

impl FromStr for Day4Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pair1, pair2) = s
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|num| {
                        println!("num: {}", num);
                        num.parse::<u8>().expect("needs to be a number")
                    })
                    .collect_tuple::<(u8, u8)>()
                    .expect("each line should be a pair")
            })
            .collect_tuple::<(_, _)>()
            .expect("each line should be a pair");

        Ok(Day4Command { pair1, pair2 })
    }
}

struct Day4(Vec<Day4Command>);

impl Day for Day4 {
    const DAY: u8 = 4;

    fn from_input(input: String) -> Self {
        let lines: Vec<Day4Command> = input
            .trim()
            .split('\n')
            .map(|line| Day4Command::from_str(line).unwrap())
            .collect();

        return Self(lines);
    }

    fn first_part(&mut self) -> String {
        let part1 = self.0.iter().fold(0, |acc, Day4Command { pair1, pair2 }| {
            if (pair1.0 >= pair2.0 && pair1.1 <= pair2.1)
                || (pair2.0 >= pair1.0 && pair2.1 <= pair1.1)
            {
                acc + 1
            } else {
                acc
            }
        });

        dbg!(part1);
        "".to_string()
    }

    fn second_part(&mut self) -> String {
        let part2 = self.0.iter().fold(0, |acc, Day4Command { pair1, pair2 }| {
            if (pair1.0 <= pair2.0 && pair2.0 <= pair1.1)
                || (pair1.0 <= pair2.1 && pair2.0 <= pair1.1)
                || (pair2.0 <= pair1.0 && pair1.0 <= pair2.1)
                || (pair2.1 <= pair1.1 && pair1.1 <= pair2.1)
            {
                acc + 1
            } else {
                acc
            }
        });

        dbg!(part2);
        "".to_string()
    }
}

fn main() -> Result<(), anyhow::Error> {
    AocSession::new(2022)?.day::<Day4>();
    Ok(())
}

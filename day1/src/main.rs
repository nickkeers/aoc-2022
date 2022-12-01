use aoc_helpers::{AocSession, Day};
use std::{convert::Infallible, str::FromStr};
use anyhow::Result;

#[derive(Debug)]
enum ItemsCommand {
    Item(i32),
    NextElf
}

struct Day1(Vec<Vec<i32>>);

impl Day for Day1 {
    const DAY: u8 = 1;
    fn from_input(input: String) -> Self {
        let lines: Vec<ItemsCommand> = input.trim().split('\n').map(| line | {
            ItemsCommand::from_str(line).unwrap()
        }).collect();

        let mut elves: Vec<Vec<i32>> = vec![];
        let mut temp_vec: Vec<i32> = vec![];

        for line in lines {
            match line {
                ItemsCommand::NextElf => {
                    elves.push(temp_vec);
                    temp_vec = vec![];
                }

                ItemsCommand::Item(calories) => {
                    temp_vec.push(calories.clone());
                }
            }
        }

        return Self(elves);
    }

    fn first_part(&mut self) -> String {
        let mut most_calories = 0;

        for elf in &self.0 {
            let total = elf.iter().sum();
            if total > most_calories {
                most_calories = total;
            }
        }

        return most_calories.to_string()
    }

    fn second_part(&mut self) -> String {        
        let mut sorted: Vec<i32> = self.0.clone().iter().map(|elf| elf.iter().sum()).collect::<Vec<i32>>();
        sorted.sort();
        sorted.iter().rev().take(3).sum::<i32>().to_string()
    }
}

impl FromStr for ItemsCommand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::NextElf),
            _ => {
                let value = s.parse::<i32>().unwrap();
                Ok(Self::Item(value))
            }
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    AocSession::new(2022)?.day::<Day1>();
    Ok(())
}


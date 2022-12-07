use aoc_helpers::{AocSession, Day};
use std::{convert::Infallible, str::FromStr, collections::{HashSet, HashMap}};
use anyhow::Result;

#[derive(Debug, Clone)]
struct Command {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Command {
    fn new(first: HashSet<char>, second: HashSet<char>) -> Self {
        Self { first, second }
    }

    // a function that given a set of letters returns the sum of their values where a-z = 1-26, A-Z=27-52
    fn sum_of_letters(letters: &Vec<char>) -> u32 {
        letters.iter().map(|c| c.to_digit(10).unwrap() - 'a'.to_digit(10).unwrap() + 1).sum()
    }

}
    
impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_at(s.len() / 2);
        let first: HashSet<char> = first.chars().collect();
        let second: HashSet<char> = second.chars().collect();

        Ok(Command::new(first, second))
    }
}

trait Extensions {
    fn common_letters(&self) -> Vec<char>;
}

impl Extensions for Command {
    fn common_letters(&self) -> Vec<char> {
        self.first.intersection(&self.second).cloned().collect()
    }
}

struct Day3(Vec<Command>);

impl Day for Day3 {
    const DAY: u8 = 3;

    fn from_input(input: String) -> Self {
        let lines: Vec<Command> = input.trim().split('\n').map(|line| {
            Command::from_str(line).unwrap()
        }).collect();

        return Self(lines);
    }

    fn first_part(&mut self) -> String {
        let mut lookup: HashMap<char, i32> = ('a'..='z').zip(1..=26).collect();
        lookup.extend(('A'..='Z').zip(27..=52));

        println!("do something with {:?} - {:?}", &self.0[0].common_letters(),  Command::sum_of_letters(&self.0[0].common_letters()));
        unimplemented!()
    }

    fn second_part(&mut self) -> String {
        unimplemented!()
    } 
}


fn main() -> Result<(), anyhow::Error> {
    AocSession::new(2022)?.day::<Day3>();
    Ok(())
}

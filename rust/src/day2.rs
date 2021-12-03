// https://adventofcode.com/2021/day/2
use std::default::Default;
use std::str::FromStr;

#[derive(Debug)]
pub enum Movement {
    Up(u32),
    Down(u32),
    Forward(u32),
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(char::is_whitespace).collect();

        if parts.len() != 2 {
            return Err("Invalid input, expecting two components!");
        }

        let distance = parts[1].parse().unwrap();

        match parts[0] {
            "up" => Ok(Movement::Up(distance)),
            "down" => Ok(Movement::Down(distance)),
            "forward" => Ok(Movement::Forward(distance)),
            _ => Err("Invalid movement specified!"),
        }
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Result<Vec<Movement>, &'static str> {
    input.lines().map(str::parse).collect()
}

#[derive(Debug, Default)]
pub struct Position {
    depth: u32,
    horizontal: u32,
}

impl Position {
    fn cross(&self) -> u32 {
        self.depth * self.horizontal
    }
}

pub trait Submarine {
    fn position(&self) -> &Position;

    fn move_sub(&mut self, movement: &Movement);

    fn move_all(&mut self, movements: &[Movement]) {
        for mvt in movements {
            self.move_sub(mvt)
        }
    }

    fn solve(&mut self, movements: &[Movement]) -> u32 {
        self.move_all(movements);
        self.position().cross()
    }
}

#[derive(Default)]
pub struct Submarine1 {
    position: Position,
}

impl Submarine for Submarine1 {
    fn move_sub(&mut self, movement: &Movement) {
        match movement {
            Movement::Up(distance) => self.position.depth -= distance,
            Movement::Down(distance) => self.position.depth += distance,
            Movement::Forward(distance) => self.position.horizontal += distance,
        }
    }

    fn position(&self) -> &Position {
        &self.position
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[Movement]) -> u32 {
    let mut sub = Submarine1::default();

    sub.solve(input)
}

#[derive(Default)]
pub struct Submarine2 {
    position: Position,
    aim: u32,
}

impl Submarine for Submarine2 {
    fn move_sub(&mut self, movement: &Movement) {
        match movement {
            Movement::Up(distance) => self.aim -= distance,
            Movement::Down(distance) => self.aim += distance,
            Movement::Forward(distance) => {
                self.position.horizontal += distance;
                self.position.depth += self.aim * distance
            }
        }
    }

    fn position(&self) -> &Position {
        &self.position
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &[Movement]) -> u32 {
    let mut sub = Submarine2::default();

    sub.solve(input)
}

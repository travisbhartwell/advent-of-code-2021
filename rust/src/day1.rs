// https://adventofcode.com/2021/day/1
use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|line| line.trim().parse()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|&x| x[1] > x[0]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> usize {
    input
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count()
}

use anyhow::Result;
use std::{fs::read_to_string, str::FromStr};
pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

const EXAMPLE_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
#[derive(Debug)]
enum Move {
    FORWARD(usize),
    UP(usize),
    DOWN(usize),
}
impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, distance)) = s.split_once(" ") {
            let distance = distance.parse().unwrap();
            Ok(match direction {
                "forward" => Move::FORWARD(distance),
                "down" => Move::DOWN(distance),
                "up" => Move::UP(distance),
                _ => panic!("Something went wrong parsing direction"),
            })
        } else {
            Err("Could not parse".to_string())
        }
    }
}
#[derive(Debug)]
struct Postion {
    horizontal: usize,
    depth: usize,
    aim: usize,
}
impl Postion {
    fn new() -> Postion {
        Postion {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn move_postion_part_one(&mut self, move_type: Move) {
        match move_type {
            Move::UP(i) => self.depth -= i,
            Move::DOWN(i) => self.depth += i,
            Move::FORWARD(i) => self.horizontal += i,
        }
    }
    fn move_postion_part_two(&mut self, move_type: Move) {
        match move_type {
            Move::UP(i) => self.aim -= i,
            Move::DOWN(i) => self.aim += i,
            Move::FORWARD(i) => {
                self.horizontal += i;
                self.depth += self.aim * i
            }
        }
    }
    fn calc_postion(&self) -> usize {
        self.horizontal * self.depth
    }
}

#[test]
fn part_one_test() {
    let mut postion = Postion::new();
    for line in EXAMPLE_INPUT.lines() {
        postion.move_postion_part_one(line.parse::<Move>().unwrap())
    }
    assert_eq!(postion.calc_postion(), 150);
}
#[test]
fn part_two_test_fold() {
    let answer = EXAMPLE_INPUT
        .lines()
        .into_iter()
        .fold(Postion::new(), |loc, dir| {
            match dir.parse::<Move>().unwrap() {
                Move::FORWARD(distance) => Postion {
                    horizontal: loc.horizontal + distance,
                    depth: loc.depth + distance * loc.aim,
                    ..loc
                },
                Move::UP(distance) => Postion {
                    aim: loc.aim - distance,
                    ..loc
                },
                Move::DOWN(distance) => Postion {
                    aim: loc.aim + distance,
                    ..loc
                },
            }
        })
        .calc_postion();
    assert_eq!(answer, 900)
}
#[test]
fn part_two_test() {
    let mut postion = Postion::new();
    for line in EXAMPLE_INPUT.lines() {
        postion.move_postion_part_two(line.parse::<Move>().unwrap())
    }
    assert_eq!(postion.calc_postion(), 900);
}
fn part_one() -> usize {
    let mut postion = Postion::new();
    for line in read_to_string("inputs/day2.txt").unwrap().lines() {
        postion.move_postion_part_one(line.parse::<Move>().unwrap())
    }
    postion.calc_postion()
}
fn part_two() -> usize {
    let mut postion = Postion::new();
    for line in read_to_string("inputs/day2.txt").unwrap().lines() {
        postion.move_postion_part_two(line.parse::<Move>().unwrap())
    }
    postion.calc_postion()
}
fn main() {
    dbg!(part_two());
}

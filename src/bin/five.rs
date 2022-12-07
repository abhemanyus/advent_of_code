use std::ops::{Deref, DerefMut};

use advent_of_code::load_file;

fn main() {
    let data = load_file("five");
    let split_index = data.find("\nmove").unwrap();
    let (stacks, moves) = data.split_at(split_index);
    let hold = Hold::try_from(stacks).unwrap();
    let crane = Crane::try_from(moves).unwrap();
}

#[derive(Clone, Copy)]
struct Crate(char);
impl TryFrom<char> for Crate {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ch @ 'A'..='Z' => Ok(Self(ch)),
            e => Err(format!("{e} is not valid Crate")),
        }
    }
}
impl Deref for Crate {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone)]
struct Stack(Vec<Crate>);
impl Deref for Stack {
    type Target = Vec<Crate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
struct Hold(Vec<Stack>);
impl Deref for Hold {
    type Target = Vec<Stack>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TryFrom<&str> for Hold {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<&str> = value.split("\n").filter(|str| str.len() > 0).collect();
        let mut hold: Vec<Stack> =
            vec![Stack(Vec::new()); lines.first().map(|f| f.len()).unwrap_or_default()];
        for line in lines.iter().rev().skip(1) {
            for (i, ch) in line.chars().enumerate().skip(1).step_by(3) {
                hold[i].push(ch.try_into()?);
            }
        }
        Ok(Self(hold))
    }
}
#[derive(Clone, Copy)]
struct Move {
    From: usize,
    To: usize,
    Amount: usize,
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = value.split(" ");
        match tokens.collect::<Vec<&str>>()[..] {
            [_, amount, _, from, _, to] => Ok(Move {
                Amount: amount
                    .parse()
                    .or(Err(format!("{amount} is not a valid amount")))?,
                From: from
                    .parse()
                    .or(Err(format!("{from} is not a valid from")))?,
                To: to.parse().or(Err(format!("{to} is not a valid to")))?,
            }),
            _ => Err(format!("{value} has incorrect tokens!")),
        }
    }
}

struct Crane(Vec<Move>);
impl Deref for Crane {
    type Target = Vec<Move>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TryFrom<&str> for Crane {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(Move::try_from)
            .collect::<Result<Vec<Move>, String>>()?;
        Ok(Self(lines))
    }
}

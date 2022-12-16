use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use advent_of_code::load_file;

fn main() {
    let data = load_file("five");
    let split_index = data.find("\nmove").unwrap();
    let (stacks, moves) = data.split_at(split_index);
    let mut hold = Hold::try_from(stacks).unwrap();
    let mut crane = Crane::try_from(moves).unwrap();
    crane.move_crates(&mut hold).unwrap();
    let top_crates = hold
        .iter()
        .map(|stack| stack.iter().last())
        .collect::<Option<Vec<_>>>()
        .unwrap();
    let top_chars = top_crates.iter().map(|c| c.0);
    println!("Ding ding, the answer is {}", String::from_iter(top_chars));
}

#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Debug)]
struct Hold(Vec<Stack>);
impl Deref for Hold {
    type Target = Vec<Stack>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Hold {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<&str> for Hold {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<&str> = value.split('\n').filter(|str| !str.is_empty()).collect();
        let mut hold: Vec<Stack> =
            vec![Stack(Vec::new()); (lines.first().map(|f| f.len()).unwrap_or_default() + 1) / 4];
        for line in lines.iter().rev().skip(1) {
            for (i, ch) in line.chars().skip(1).step_by(4).enumerate() {
                if ch != ' ' {
                    hold[i].push(ch.try_into()?);
                }
            }
        }
        Ok(Self(hold))
    }
}
#[derive(Clone, Copy, Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = value.split(' ');
        match tokens.collect::<Vec<&str>>()[..] {
            [_, amount, _, from, _, to] => Ok(Move {
                amount: amount
                    .parse()
                    .or(Err(format!("{amount} is not a valid amount")))?,
                from: from
                    .parse()
                    .or(Err(format!("{from} is not a valid from")))?,
                to: to.parse().or(Err(format!("{to} is not a valid to")))?,
            }),
            _ => Err(format!("{value} has incorrect tokens!")),
        }
    }
}

#[derive(Debug)]
struct Crane(VecDeque<Move>);
impl Deref for Crane {
    type Target = VecDeque<Move>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Crane {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<&str> for Crane {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(Move::try_from)
            .collect::<Result<VecDeque<Move>, String>>()?;
        Ok(Self(lines))
    }
}
impl Crane {
    fn move_crates(&mut self, hold: &mut Hold) -> Result<(), String> {
        while let Some(current_move) = self.pop_front() {
            let from_len = hold[current_move.from - 1].len();
            let (first, second) =
                hold[current_move.from - 1].split_at(from_len - current_move.amount);
            let first = Stack(first.to_vec());
            let mut picked_crates = second.to_vec();
            hold[current_move.to - 1].append(&mut picked_crates);
            hold[current_move.from - 1] = first;
        }
        Ok(())
    }
}

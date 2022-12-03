use std::{collections::BTreeSet, str::FromStr};

use advent_of_code::load_file;

fn main() {
    let data = load_file("three");
    let list = data
        .split("\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<Bag>, String>>()
        .unwrap();
    let shared_items = list
        .iter()
        .map(|bag| bag.shared_item())
        .collect::<Option<Vec<Item>>>()
        .unwrap();
    let priorities: u32 = shared_items.iter().map(|i| i.value()).sum();
    println!("Ding ding, the answer is {priorities}");
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item(char);

impl Item {
    fn value(self) -> u32 {
        match self.0 {
            'a'..='z' => self.0 as u32 - 96,
            'A'..='Z' => self.0 as u32 - 64 + 26,
            _ => 0,
        }
    }
}

impl TryFrom<char> for Item {
    type Error = String;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            c @ 'A'..='z' => Ok(Self(c)),
            v => Err(format!("{v} is not valid Item")),
        }
    }
}

struct Bag {
    first_pocket: Vec<Item>,
    second_pocket: Vec<Item>,
}

impl Bag {
    fn shared_item(&self) -> Option<Item> {
        let first_pocket = BTreeSet::from_iter(self.first_pocket.iter());
        let second_pocket = BTreeSet::from_iter(self.second_pocket.iter());
        let shared_item = first_pocket
            .intersection(&second_pocket)
            .next()
            .cloned()
            .cloned();
        shared_item
    }
}

impl FromStr for Bag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_pocket, second_pocket) = s.split_at(s.len() / 2);
        Ok(Self {
            first_pocket: first_pocket
                .chars()
                .map(|ch| Item::try_from(ch))
                .collect::<Result<Vec<Item>, String>>()?,
            second_pocket: second_pocket
                .chars()
                .map(|ch| Item::try_from(ch))
                .collect::<Result<Vec<Item>, String>>()?,
        })
    }
}

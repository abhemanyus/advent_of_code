#![feature(iter_array_chunks)]
use std::collections::BTreeSet;

use advent_of_code::load_file;

fn main() {
    let data = load_file("three");
    let list = data
        .split('\n')
        .array_chunks::<3>()
        .map(|s| (s[0], s[1], s[2]).try_into())
        .collect::<Result<Vec<Group>, String>>()
        .unwrap();
    let shared_badges = list
        .iter()
        .map(|group| group.shared_badge())
        .collect::<Option<Vec<Item>>>()
        .unwrap();
    let priorities: u32 = shared_badges.iter().map(|i| i.value()).sum();
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

struct Bag(Vec<Item>);

impl Bag {
    fn shared_item(&self) -> Option<Item> {
        let (first_pocket, second_pocket) = self.0.split_at(self.0.len() / 2);
        let first_pocket = BTreeSet::from_iter(first_pocket.iter());
        let second_pocket = BTreeSet::from_iter(second_pocket.iter());
        let shared_item = first_pocket
            .intersection(&second_pocket)
            .next()
            .cloned()
            .cloned();
        shared_item
    }
}

impl TryFrom<&str> for Bag {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.chars()
                .map(Item::try_from)
                .collect::<Result<Vec<Item>, String>>()?,
        ))
    }
}

struct Group(Bag, Bag, Bag);

impl TryFrom<(&str, &str, &str)> for Group {
    type Error = String;
    fn try_from(value: (&str, &str, &str)) -> Result<Self, Self::Error> {
        Ok(Group(
            value.0.try_into()?,
            value.1.try_into()?,
            value.2.try_into()?,
        ))
    }
}

impl Group {
    fn shared_badge(&self) -> Option<Item> {
        let one = BTreeSet::from_iter((self.0).0.iter());
        let two = BTreeSet::from_iter((self.1).0.iter());
        let three = BTreeSet::from_iter((self.2).0.iter());
        let first_intersect: BTreeSet<&&Item> = one.intersection(&two).collect();
        let second_intersect = two.intersection(&three).collect();
        let shared_badge = first_intersect
            .intersection(&second_intersect)
            .next()
            .cloned()
            .cloned()
            .cloned();
        shared_badge
    }
}

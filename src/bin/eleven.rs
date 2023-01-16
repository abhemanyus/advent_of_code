#![feature(iter_array_chunks)]
use std::collections::VecDeque;

use advent_of_code::load_file;

fn gcd(x: u64, y: u64) -> u64 {
    let mut c = x;
    let mut d = y;
    while d > 0 {
        (c, d) = (d, c % d);
    }
    return c;
}

fn lcm(x: u64, y: u64) -> u64 {
    return x * y / gcd(x, y);
}

fn lcmm(ls: &[u64]) -> u64 {
    ls.iter().fold(1u64, |last, cur| lcm(last, *cur))
}

#[test]
fn get_lcm() {
    let lcm = lcmm(&[1, 2, 3]);
    assert_eq!(lcm, 6u64);
}

fn main() {
    let data = load_file!("eleven");
    let mut game: Game = data.as_str().try_into().unwrap();
    println!("Monkeys {}", game.monkeys.len());
    println!("LCM {}", game.lcm);
    for _ in 0..10_000 {
        game.play();
        // for (i, monke) in game.monkeys.iter().enumerate() {
        //     println!("Monke {i}: {:?}", monke.items);
        // }
    }
    game.monkeys.sort_by_key(|monke| monke.inspected);
    // println!("{:?}", game.monkeys);
    let len = game.monkeys.len();
    let first = game.monkeys[len - 1].inspected;
    let second = game.monkeys[len - 2].inspected;
    println!(
        "Monkeys {:?}",
        game.monkeys
            .iter()
            .map(|monke| monke.inspected)
            .collect::<Vec<_>>()
    );
    println!("Ding dong, the answer is {}!", first * second);
}

struct Game {
    round: usize,
    lcm: u64,
    monkeys: Vec<Monkey>,
}

impl Game {
    fn play(&mut self) {
        self.round += 1;
        for i in 0..self.monkeys.len() {
            while let Some((target, item)) = self.monkeys[i].throw(self.lcm) {
                let catcher = self.monkeys.get_mut(target).unwrap();
                catcher.catch(item);
            }
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let monkeys: Vec<Monkey> = value
            .split("\n\n")
            .map(|chunk| Monkey::try_from(chunk))
            .collect::<Result<_, _>>()?;
        let lcm = lcmm(
            &monkeys
                .iter()
                .map(|monke| monke.div_test)
                .collect::<Vec<_>>(),
        );
        Ok(Self {
            round: 0,
            lcm: lcm,
            monkeys: monkeys,
        })
    }
}

type MonkeyNumber = usize;

#[derive(Debug)]
struct Action {
    True: MonkeyNumber,
    False: MonkeyNumber,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    div_test: u64,
    action: Action,
    worry: Calculation,
    inspected: usize,
}

impl Monkey {
    fn throw(&mut self, lcm: u64) -> Option<(MonkeyNumber, Item)> {
        let item = self.items.pop_front()?;
        let worry = self.worry.new_worry(item) % lcm;
        self.inspected += 1;
        let monke = if worry % self.div_test == 0 {
            self.action.True
        } else {
            self.action.False
        };
        Some((monke, worry))
    }

    fn catch(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

impl TryFrom<&str> for Monkey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let [_, items, operation, test, true_action, false_action]: [&str; 6] = value
            .split("\n")
            .map(|l| l.trim())
            .array_chunks()
            .next()
            .ok_or(format!("{} is not a valid monkey", value))?;
        let items = items
            .split_once(": ")
            .ok_or(format!("oops"))?
            .1
            .split(",")
            .map(|token| token.trim().parse())
            .collect::<Result<_, _>>()
            .or(Err(format!("unable to parse items")))?;
        let operation = operation
            .split_once(": ")
            .ok_or(format!("unable to parse operation"))?
            .1
            .try_into()?;
        let test = test
            .split(" ")
            .last()
            .ok_or(format!("unable to parse test"))?
            .parse()
            .or(Err(format!("unable to parse test")))?;
        let true_action = true_action
            .split(" ")
            .last()
            .ok_or(format!("unable to parse true action"))?
            .parse()
            .or(Err(format!("unable to parse true action")))?;
        let false_action = false_action
            .split(" ")
            .last()
            .ok_or(format!("unable to parse false action"))?
            .parse()
            .or(Err(format!("unable to parse false action")))?;
        let monke = Monkey {
            items,
            div_test: test,
            inspected: 0,
            action: Action {
                True: true_action,
                False: false_action,
            },
            worry: operation,
        };
        Ok(monke)
    }
}

#[test]
fn parse_game() {
    let data = load_file!("eleven");
    let game: Game = data.as_str().try_into().unwrap();
    dbg!(game.monkeys);
}

#[derive(Debug)]
enum Calculation {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Square,
}

impl Calculation {
    fn new_worry(&self, level: WorryLevel) -> WorryLevel {
        match self {
            Calculation::Add(val) => level + val,
            Calculation::Multiply(val) => level * val,
            Calculation::Square => level * level,
        }
    }
}

enum Operator {
    Add,
    Multiply,
}
impl TryFrom<&str> for Calculation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let [_, _, _, op, opand]: [&str; 5] = value
            .split(" ")
            .array_chunks()
            .next()
            .ok_or(format!("{} is not a calc", value))?;
        let op = match op {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => Err(format!("{} is not a valid operator", op))?,
        };
        let calc = match opand {
            "old" => Calculation::Square,
            num => {
                let num = num.parse().or(Err(format!("{} is not a number", num)))?;
                match op {
                    Operator::Add => Calculation::Add(num),
                    Operator::Multiply => Calculation::Multiply(num),
                }
            }
        };
        Ok(calc)
    }
}

#[test]
fn parse_calc() {
    let stmt = "new = old * old";
    let calc = stmt.try_into().unwrap();
    assert!(matches!(calc, Calculation::Square));
}

type WorryLevel = u64;

type Item = WorryLevel;

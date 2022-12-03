use std::str::FromStr;

use advent_of_code::load_file;

fn main() {
    let data = load_file("two");
    let score: u32 = data
        .split("\n")
        .map(|plan| {
            let plan: Plan = plan.parse().unwrap();
            let game: Game = plan.into();
            game.score()
        })
        .sum();
    println!("Ding ding, the answer is {score}!");
}

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}
struct ElfMove(Move);
struct PlayerMove(Move);
impl FromStr for ElfMove {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let game_move = match value {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            _ => return Err(format!("{value} is not valid elf move")),
        };
        Ok(Self(game_move))
    }
}
impl FromStr for PlayerMove {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let game_move = match value {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissor,
            _ => return Err(format!("{value} is not valid player move")),
        };
        Ok(Self(game_move))
    }
}

struct Game(ElfMove, PlayerMove);

impl FromStr for Game {
    type Err = String;
    fn from_str(game: &str) -> Result<Self, Self::Err> {
        let strings: Vec<&str> = game.split(" ").collect();
        let (elf, player) = (strings[0], strings[1]);
        Ok(Self(elf.parse()?, player.parse()?))
    }
}

#[derive(Clone, Copy)]
enum EndGame {
    Lose,
    Win,
    Draw,
}
impl FromStr for EndGame {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let game_move = match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => return Err(format!("{value} is not valid end game")),
        };
        Ok(game_move)
    }
}

impl EndGame {
    fn score(self) -> u32 {
        match self {
            EndGame::Lose => 0,
            EndGame::Draw => 3,
            EndGame::Win => 6,
        }
    }

    fn get_move(self, opponent: Move) -> Move {
        match self {
            EndGame::Lose => opponent.win(),
            EndGame::Win => opponent.loose(),
            EndGame::Draw => opponent,
        }
    }
}

impl Move {
    fn loose(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        }
    }
    fn win(self) -> Move {
        match self {
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
            Move::Rock => Move::Scissor,
        }
    }
    fn end_game(self, opponent: Self) -> EndGame {
        if self == opponent {
            return EndGame::Draw;
        }
        if self.loose() == opponent {
            return EndGame::Lose;
        }
        EndGame::Win
    }

    fn score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
}

impl Game {
    fn score(&self) -> u32 {
        let game_result = (self.1).0.end_game(self.0 .0);
        let score = game_result.score() + (self.1).0.score();
        return score;
    }
}

impl From<Plan> for Game {
    fn from(plan: Plan) -> Self {
        let player_move = (plan.1).get_move((plan.0).0);
        Game(plan.0, PlayerMove(player_move))
    }
}

struct Plan(ElfMove, EndGame);
impl FromStr for Plan {
    type Err = String;
    fn from_str(game: &str) -> Result<Self, Self::Err> {
        let strings: Vec<&str> = game.split(" ").collect();
        let (elf, end_game) = (strings[0], strings[1]);
        Ok(Self(elf.parse()?, end_game.parse()?))
    }
}

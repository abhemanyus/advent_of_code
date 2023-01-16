use std::{collections::BTreeSet, fmt::Display, ops::Add};

use advent_of_code::load_file;

fn main() {
    let data = load_file!("nine");
    let move_list: Vec<Move> = data.split('\n').map(Move::from).collect();
    let mut board = Board::default();
    // println!("{}", &board);
    for mv in move_list {
        board.exec_move(mv);
    }
    println!("Total unique positions: {}", board.move_set.len());
}

#[derive(Debug)]
enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let (mv, val) = value.split_once(' ').unwrap();
        let val: u32 = val.parse().unwrap();
        match mv {
            "U" => Self::Up(val),
            "D" => Self::Down(val),
            "L" => Self::Left(val),
            "R" => Self::Right(val),
            _ => panic!("invalid move {mv}"),
        }
    }
}

impl Move {
    fn direction(&self) -> Position {
        match self {
            Move::Up(_) => Position { x: 0, y: 1 },
            Move::Down(_) => Position { x: 0, y: -1 },
            Move::Left(_) => Position { x: -1, y: 0 },
            Move::Right(_) => Position { x: 1, y: 0 },
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Move::Up(v) => *v,
            Move::Down(v) => *v,
            Move::Left(v) => *v,
            Move::Right(v) => *v,
        }
    }

    fn decrease_magnitude(&self) -> Option<Move> {
        if self.magnitude() == 1 {
            return None;
        }
        Some(match self {
            Move::Up(v) => Move::Up(v - 1),
            Move::Down(v) => Move::Down(v - 1),
            Move::Left(v) => Move::Left(v - 1),
            Move::Right(v) => Move::Right(v - 1),
        })
    }
}

#[test]
fn move_test() {
    let mv = Move::Up(1);
    let mv = mv.decrease_magnitude().unwrap();
    assert!(matches!(mv.decrease_magnitude(), None));
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Position {
    fn exec_move(self, mv: Move) -> (Position, Option<Move>) {
        let dir = mv.direction();
        let pos = self + dir;
        (pos, mv.decrease_magnitude())
    }

    fn update_pos(&mut self, head: Position) {
        let (dx, dy) = (head.x - self.x, head.y - self.y);
        if dx.abs() > 1 {
            self.x += dx.signum();
            self.y += dy.signum();
        } else if dy.abs() > 1 {
            self.y += dy.signum();
            self.x += dx.signum();
        }
    }
}

#[derive(Default, Debug)]
struct Board {
    rope: [Position; 10],
    move_set: BTreeSet<Position>,
}

impl Board {
    fn exec_move(&mut self, mv: Move) {
        let (head, mv) = self.rope[0].exec_move(mv);
        self.rope[0] = head;
        for i in 1..10 {
            self.rope[i].update_pos(self.rope[i - 1]);
        }
        self.move_set.insert(self.rope[9]);
        // println!("{}", &self);
        if let Some(mv) = mv {
            self.exec_move(mv);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..6 {
                let pos = Position { x, y };
                let (i, _) = self.rope.iter().enumerate().find(|f| *f.1 == pos).unzip();
                if let Some(i) = i {
                    write!(f, "{i}")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

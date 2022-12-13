use std::{collections::BTreeSet, fmt::Display, ops::Add};

use advent_of_code::load_file;

fn main() {
    let data = load_file("nine");
    let move_list: Vec<Move> = data.split("\n").map(Move::from).collect();
    let mut board = Board::default();
    for mv in move_list {
        println!("{:?}", mv);
        board.exec_move(mv);
        println!("{}", &board);
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
        let (mv, val) = value.split_once(" ").unwrap();
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
}

#[derive(Default, Debug)]
struct Board {
    head: Position,
    tail: Position,
    move_set: BTreeSet<Position>,
}

impl Board {
    fn exec_move(&mut self, mv: Move) {
        let (new_head, new_mv) = self.head.exec_move(mv);
        self.head = new_head;
        let (dx, dy) = (self.head.x - self.tail.x, self.head.y - self.tail.y);
        if dx.abs() > 1 {
            self.tail.x += dx.signum();
            self.tail.y += dy.signum();
        }
        if dy.abs() > 1 {
            self.tail.y += dy.signum();
            self.tail.x += dx.signum();
        }
        self.move_set.insert(self.tail);
        if let Some(mv) = new_mv {
            self.exec_move(mv);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "H: [{}, {}], T: [{},{}]",
            self.head.x, self.head.y, self.tail.x, self.tail.y
        )
    }
}

#[test]
fn board_test() {
    let mv = Move::Up(1);
    let mut board = Board::default();
    board.exec_move(mv);
    assert!(matches!(board.head, Position { x: 0, y: 1 }));
}

#[test]
fn tail_test() {
    let mut board = Board::default();
    board.exec_move(Move::Up(1));
    assert_eq!(board.move_set.len(), 1);
    assert!(matches!(board.tail, Position { x: 0, y: 0 }));
    board.exec_move(Move::Right(1));
    assert_eq!(board.move_set.len(), 1);
    assert!(matches!(board.tail, Position { x: 0, y: 0 }));
    board.exec_move(Move::Right(1));
    assert_eq!(board.move_set.len(), 2);
    assert!(matches!(board.tail, Position { x: 1, y: 0 }));
}

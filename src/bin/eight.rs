use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use advent_of_code::load_file;

fn main() {
    let data = load_file("eight");
    let mut grid: Grid = data.as_str().into();
    // grid[(1,1)].visible = false;
    grid.visibility_check();
    // println!("Grid:\n{}", &grid);
    let score = grid.cells.iter().max_by_key(|tree| tree.score).unwrap();
    println!("Max score: {}", score.score);
}

#[derive(Clone, Copy)]
struct Tree {
    height: i8,
    visible: bool,
    score: i32,
}
impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visible {
            write!(f, " {} ", self.height)
        } else {
            write!(f, "[{}]", self.height)
        }
    }
}
impl From<char> for Tree {
    fn from(value: char) -> Self {
        return Tree {
            height: (value as i8) & 0x0F,
            visible: false,
            score: 1,
        };
    }
}
struct Grid {
    cells: Vec<Tree>,
    height: usize,
    width: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}\n", self.width, self.height)?;
        for (index, tree) in self.cells.iter().enumerate() {
            write!(f, "{} ", tree)?;
            if (index + 1) % self.width == 0 {
                write!(f, "\n")?;
            }
        }
        write!(f, "\n")
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tree;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.cells[index.0 + index.1 * self.width];
    }
}
impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        return &mut self.cells[index.0 + index.1 * self.width];
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let lines = value.split("\n").collect::<Vec<&str>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut grid = Grid {
            cells: Vec::with_capacity(height * width),
            height: height,
            width: width,
        };
        for line in lines {
            for ch in line.chars() {
                grid.cells.push(ch.into())
            }
        }
        grid
    }
}

impl Grid {
    fn visibility_check(&mut self) {
        self.check_north();
        self.check_east();
        self.check_south();
        self.check_west();
    }
    fn check_north(&mut self) {
        for x in 0..self.width {
            let mut max_height = -1;
            let mut max_dist = 0;
            for y in 0..self.height {
                let tree = self.index_mut((x, y));
                if tree.height > max_height {
                    tree.visible = true;
                    max_height = tree.height;
                    tree.score *= (y - max_dist) as i32;
                    max_dist = y;
                }
            }
        }
    }
    fn check_east(&mut self) {
        for y in 0..self.height {
            let mut max_height = -1;
            let mut max_dist = self.width;
            for x in (0..self.width).rev() {
                let tree = self.index_mut((x, y));
                if tree.height > max_height {
                    tree.visible = true;
                    max_height = tree.height;
                    tree.score *= (max_dist - x) as i32;
                    max_dist = x;
                }
            }
        }
    }
    fn check_south(&mut self) {
        for x in 0..self.width {
            let mut max_height = -1;
            let mut max_dist = self.height;
            for y in (0..self.height).rev() {
                let tree = self.index_mut((x, y));
                if tree.height > max_height {
                    tree.visible = true;
                    max_height = tree.height;
                    tree.score *= (max_dist - y) as i32;
                    max_dist = y;
                }
            }
        }
    }
    fn check_west(&mut self) {
        for y in 0..self.height {
            let mut max_height = -1;
            let mut max_dist = 0;
            for x in 0..self.width {
                let tree = self.index_mut((x, y));
                if tree.height > max_height {
                    tree.visible = true;
                    max_height = tree.height;
                    tree.score *= (x - max_dist) as i32;
                    max_dist = x;
                }
            }
        }
    }
}

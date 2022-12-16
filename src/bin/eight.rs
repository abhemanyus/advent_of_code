use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use advent_of_code::load_file;

fn main() {
    let data = load_file("eight");
    let mut grid: Grid = data.as_str().into();
    grid.set_scores();
    let score = grid.cells.iter().max_by_key(|tree| tree.score).unwrap();
    println!("Max: {}: {}", score, score.score);
    // println!("Grid: {}\nScore: {}", grid, grid.calculate_score((2, 3)));
}

#[derive(Clone, Copy)]
struct Tree {
    height: i8,
    visible: bool,
    score: usize,
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
        Tree {
            height: (value as i8) & 0x0F,
            visible: false,
            score: 1,
        }
    }
}
struct Grid {
    cells: Vec<Tree>,
    height: usize,
    width: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}x{}", self.width, self.height)?;
        for (index, tree) in self.cells.iter().enumerate() {
            write!(f, "{tree} ")?;
            if (index + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tree;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.0 + index.1 * self.width]
    }
}
impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[index.0 + index.1 * self.width]
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let lines = value.split('\n').collect::<Vec<&str>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut grid = Grid {
            cells: Vec::with_capacity(height * width),
            height,
            width,
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
    fn set_scores(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let score = self.calculate_score((x, y));
                self.index_mut((x, y)).score = score;
            }
        }
    }
    fn calculate_score(&self, index: (usize, usize)) -> usize {
        self.calc_north(index)
            * self.calc_south(index)
            * self.calc_east(index)
            * self.calc_west(index)
    }
    fn calc_north(&self, index: (usize, usize)) -> usize {
        let tree = self.index(index);
        let mut steps = 0;
        for y in (0..index.1).rev() {
            steps += 1;
            if tree.height <= self.index((index.0, y)).height {
                return steps;
            }
        }
        steps
    }
    fn calc_east(&self, index: (usize, usize)) -> usize {
        let tree = self.index(index);
        let mut steps = 0;
        for x in (index.0 + 1)..self.width {
            steps += 1;
            if tree.height <= self.index((x, index.1)).height {
                return steps;
            }
        }
        steps
    }
    fn calc_south(&self, index: (usize, usize)) -> usize {
        let tree = self.index(index);
        let mut steps = 0;
        for y in (index.1 + 1)..self.height {
            steps += 1;
            if tree.height <= self.index((index.0, y)).height {
                return steps;
            }
        }
        steps
    }
    fn calc_west(&self, index: (usize, usize)) -> usize {
        let tree = self.index(index);
        let mut steps = 0;
        for x in (0..index.0).rev() {
            steps += 1;
            if tree.height <= self.index((x, index.1)).height {
                return steps;
            }
        }
        steps
    }
}

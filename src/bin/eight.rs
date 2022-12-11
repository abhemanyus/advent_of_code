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
    println!("Grid:\n{}", grid);
}

#[derive(Clone, Copy)]
struct Tree {
    height: u8,
    visible: bool,
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
            height: (value as u8) & 0x0F,
            visible: true,
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
        for x in 0..self.width {
            for y in 0..self.height {
                self.index_mut((x, y)).visible = self.news((x, y));
            }
        }
    }
    fn news(&self, index: (usize, usize)) -> bool {
        self.check_north(index)
        // || self.check_east(index)
        // || self.check_south(index)
        // || self.check_west(index)
    }
    fn check_north(&self, index: (usize, usize)) -> bool {
        let curr_tree = self.index(index);
        if index.1 == 0 {
            return true;
        };
        for y in 0..=index.1 {
            let tree = self[(index.0, y)];
            if curr_tree.height > tree.height {
                return true;
            }
        }
        return false;
    }
    fn check_east(&self, index: (usize, usize)) -> bool {
        let curr_tree = self.index(index);
        if index.0 == self.width - 1 {
            return true;
        };
        for x in index.0..self.width {
            let tree = self[(x, index.1)];
            if curr_tree.height > tree.height {
                return true;
            }
        }
        return false;
    }
    fn check_south(&self, index: (usize, usize)) -> bool {
        let curr_tree = self.index(index);
        if index.1 == self.height - 1 {
            return true;
        };
        for y in index.0..self.height {
            let tree = self[(index.0, y)];
            if curr_tree.height > tree.height {
                return true;
            }
        }
        return false;
    }
    fn check_west(&self, index: (usize, usize)) -> bool {
        let curr_tree = self.index(index);
        if index.0 == 0 {
            return true;
        };
        for x in 0..=index.0 {
            let tree = self[(x, index.1)];
            if curr_tree.height > tree.height {
                return true;
            }
        }
        return false;
    }
}

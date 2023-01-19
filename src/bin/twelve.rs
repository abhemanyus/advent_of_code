#![feature(generic_const_exprs)]
use std::fmt::Display;
use advent_of_code::load_file;

fn main() {
    let data = load_file!("twelve");
    let graph: Graph<61, 41> = data.as_str().into();
    let (matrix, start, end) = graph.matrix();
    let path = dijkstra(matrix, start, end);
    println!("Path: {:?}", path.len());
}

type Tile = [usize; 2];
type Height = u8;
struct Graph<const X: usize, const Y: usize> {
    start: Tile,
    end: Tile,
    tiles: [[Height; X]; Y],
}

impl<const X: usize, const Y: usize> Graph<X, Y> {
    fn new() -> Self {
        Self {
            start: [0, 0],
            end: [3, 3],
            tiles: [[0; X]; Y],
        }
    }

    fn matrix(&self) -> ([[bool; X*Y]; X*Y], usize, usize) {
        let mut matrix = [[false; X*Y]; X*Y];
        for y in 0..Y {
            for x in 0..X {
                let this_node = Self::tile_to_node([x, y]);
                for neighbour in self.neighbours([x, y]) {
                    let that_node = Self::tile_to_node(neighbour);
                    matrix[this_node][that_node] = true;
                }
            }
        }
        (matrix, Self::tile_to_node(self.start), Self::tile_to_node(self.end))
    }

    fn tile_to_node(tile: Tile) -> usize {
        tile[1] * X + tile[0]
    }

    fn neighbours(&self, tile: Tile) -> Vec<Tile> {
        let mut neighbours = Vec::new();
        match tile[0] {
            0 => {
                if self.can_move(tile, [1, tile[1]]) {
                    neighbours.push([1, tile[1]])
                }
            }
            x if x == X - 1 => {
                if self.can_move(tile, [x - 1, tile[1]]) {
                    neighbours.push([x - 1, tile[1]])
                }
            }
            x => {
                if self.can_move(tile, [x - 1, tile[1]]) {
                    neighbours.push([x - 1, tile[1]])
                };
                if self.can_move(tile, [x + 1, tile[1]]) {
                    neighbours.push([x + 1, tile[1]])
                }
            }
        };
        match tile[1] {
            0 => {
                if self.can_move(tile, [tile[0], 1]) {
                    neighbours.push([tile[0], 1])
                }
            }
            y if y == Y - 1 => {
                if self.can_move(tile, [tile[0], y - 1]) {
                    neighbours.push([tile[0], y - 1])
                }
            }
            y => {
                if self.can_move(tile, [tile[0], y - 1]) {
                    neighbours.push([tile[0], y - 1])
                }
                if self.can_move(tile, [tile[0], y + 1]) {
                    neighbours.push([tile[0], y + 1])
                }
            }
        };
        neighbours
    }

    fn height(&self, tile: Tile) -> Height {
        self.tiles[tile[1]][tile[0]]
    }

    fn can_move(&self, from: Tile, to: Tile) -> bool {
        let from_height = self.height(from);
        let to_height = self.height(to);
        if to_height <= from_height + 1 {
            return true;
        }
        false
    }
}

#[test]
fn neighbours() {
    let data = load_file!("twelve");
    let graph: Graph<8, 5> = data.as_str().into();
    let neigbours = graph.neighbours([2, 1]);
    dbg!(neigbours);
}

impl<const X: usize, const Y: usize> Display for Graph<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Y {
            for x in 0..X {
                let tile = self.tiles[y][x];
                if [x, y] == self.start {
                    write!(f, "|{:2}|", tile)?;
                } else if [x, y] == self.end {
                    write!(f, "({:2})", tile)?;
                } else {
                    write!(f, "[{:2}]", tile)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<const X: usize, const Y: usize> From<&str> for Graph<X, Y> {
    fn from(value: &str) -> Self {
        let mut graph = Self::new();
        for (y, line) in value.split("\n").enumerate() {
            for (x, char) in line.chars().enumerate() {
                let height = match char {
                    'S' => {
                        graph.start = [x, y];
                        0
                    }
                    'E' => {
                        graph.end = [x, y];
                        25
                    }
                    char => char as Height - 'a' as Height,
                };
                graph.tiles[y][x] = height;
            }
        }
        graph
    }
}

fn dijkstra<const N: usize>(matrix: [[bool; N]; N], start: usize, end: usize) -> Vec<usize> {
    let mut visited = [false; N];
    let mut dist = [usize::MAX; N];
    let mut path = [usize::MAX; N];
    dist[start] = 0;
    let mut current = start;
    loop {
        for neighbor in 0..N {
            if neighbor != current && matrix[current][neighbor] && visited[neighbor] == false {
                let new_dist = dist[current] + 1;
                if dist[neighbor] > new_dist {
                    dist[neighbor] = new_dist;
                    path[neighbor] = current;
                }
            }
        }
        visited[current] = true;
        if current == end {break;}
        (current, _) = dist
            .iter()
            .enumerate()
            .filter(|(node, _)| visited[*node] == false)
            .min_by_key(|(_, dist)| *dist)
            .unwrap();
    }
    let mut path_vec = Vec::new();
    current = end;
    while current != usize::MAX {
        path_vec.push(current);
        current = path[current];
    };
    path_vec.reverse();
    path_vec
    
}

#[test]
fn dijkstra_test() {
    let matrix = [
        [false, true, false, false, false, false, false, true, false],
        [true, false, false, false, false, false, false, true, false],
        [false, false, false, true, false, true, false, false, true],
        [false, false, true, false, true, true, false, false, false],
        [false, false, false, true, false, true, false, false, false],
        [false, false, true, true, true, false, true, false, false],
        [false, false, false, false, false, true, false, false, true],
        [true, true, false, false, false, false, false, false, true],
        [false, false, true, false, false, false, true, true, false],
    ];
    let path = dijkstra(matrix, 0, 4);
    dbg!(path);
}

use std::collections::{HashMap, HashSet};
use std::sync::mpsc::RecvTimeoutError::Timeout;

#[derive(Clone, PartialEq)]
enum TileColor {
    White,
    Black
}

impl TileColor {
    fn flip(&self) -> TileColor {
        use TileColor::*;
        match self {
            Black => White,
            White => Black,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct CubeCoordinates {
    x: i32,
    y: i32,
    z: i32,
}

enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    fn preprocess_directions(directions: &str) -> String {
        directions.replace(
            "se", "!"
        ).replace(
            "sw", "@"
        ).replace(
            "nw", "#"
        ).replace(
            "ne", "$"
        )
    }

    fn from_char(c: char) -> Direction {
        use Direction::*;
        match c {
            'e' => E,
            '!' => SE,
            '@' => SW,
            'w' => W,
            '#' => NW,
            '$' => NE,
            _ => panic!("should not be here, {}", c),
        }
    }
}

impl CubeCoordinates {
    fn new(triplet: (i32, i32, i32)) -> CubeCoordinates {
        let (x, y, z) = triplet;
        CubeCoordinates { x, y, z }
    }

    fn move_to(&self, direction: &Direction) -> CubeCoordinates {
        use Direction::*;
        CubeCoordinates::new(
            match direction {
                E => (self.x + 1, self.y - 1, self.z),
                SE => (self.x, self.y - 1, self.z + 1),
                SW => (self.x - 1, self.y, self.z + 1),
                W => (self.x - 1, self.y + 1, self.z),
                NW => (self.x, self.y + 1, self.z - 1),
                NE => (self.x + 1, self.y, self.z - 1),
            }
        )
    }

    fn neighbours(&self) -> Vec<CubeCoordinates> {
        use Direction::*;
        [E, SE, SW, W, NW, NE]
            .iter()
            .map(|v| self.move_to(v))
            .collect()
    }
}

fn convay(visited_tiles: HashMap<CubeCoordinates, TileColor>) -> HashMap<CubeCoordinates, TileColor>{
    let mut result= visited_tiles.iter().fold(HashMap::new()|acc, (c, tile)| {
       if *tile == TileColor::Black {
           acc.insert(c, tile)
       } else {
           acc
       }
    });

    let mut pool = HashMap::new();
    let mut seen = HashSet::new();

    while !pool.is_empty() {
        let black_neighbours=  coordinates
            .neighbours()
            .iter()
            .flat_map(|n| visited_tiles.get(n).or(Some(&TileColor::White)))
            .filter(|c| **c == TileColor::Black)
            .count();

        let new_tile_color = match color {
            TileColor::Black => if black_neighbours == 0 || black_neighbours > 2 { TileColor::White } else { TileColor::Black },
            TileColor::White => if black_neighbours == 2 { TileColor::Black } else { TileColor::White },
        };

        result.insert(coordinates.clone(), new_tile_color);
    }

    result
}

fn main() {
    let instructions = std::fs::read_to_string("inputs/input24").unwrap();

    let mut black_tiles = 0;
    let mut visited_tiles: HashMap<CubeCoordinates, TileColor> = HashMap::new();

    for i in instructions.lines() {
        let processed = Direction::preprocess_directions(i);

        let cur = processed
            .chars()
            .fold(
                CubeCoordinates::new((0, 0, 0)),
                |acc, c| acc.move_to(&Direction::from_char(c))
            );

        if let Some(color) = visited_tiles.get(&cur) {
            if *color == TileColor::Black {
                black_tiles -= 1;
            }

            visited_tiles.insert(cur.clone(), color.flip());
        } else {
            visited_tiles.insert(cur.clone(), TileColor::Black);
            black_tiles += 1;
        }
    }

    println!("black tiles {}", black_tiles);
}
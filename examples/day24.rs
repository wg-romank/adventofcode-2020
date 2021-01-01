use std::collections::HashMap;

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

#[derive(Clone, PartialEq, Eq, Hash)]
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
    fn string_to_char(directions: &str) -> String {
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

    fn move_to(self, direction: Direction) -> CubeCoordinates {
        use Direction::*;
        CubeCoordinates::new(
            match direction {
                E => (self.x - 1, self.y + 1, self.z),
                SE => (self.x - 1, self.y, self.z + 1),
                SW => (self.x, self.y - 1, self.z + 1),
                W => (self.x + 1, self.y - self.z, self.z),
                NW => (self.x + 1, self.y, self.z - 1),
                NE => (self.x, self.y + 1, self.z - 1),
            }
        )
    }
}

fn main() {
    let instructions = std::fs::read_to_string("inputs/input24").unwrap();
    let mut visited_tiles: HashMap<CubeCoordinates, TileColor> = HashMap::new();

    // key - field representation
    let reference_coordinate = CubeCoordinates::new((0, 0, 0));

    let mut black_tiles = 0;

    for i in instructions.lines() {
        let processed = Direction::string_to_char(i);
        let mut cur = reference_coordinate.clone();
        for c in processed.chars() {
            let dir = Direction::from_char(c);
            cur = cur.move_to(dir);

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
    }

    println!("black tiles {}", black_tiles);
}
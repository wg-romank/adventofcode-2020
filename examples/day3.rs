#[derive(PartialEq)]
enum Tile {
    Tree,
    Free,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Tree,
            '.' => Tile::Free,
            _ => panic!("Invalid tile descriptor {}", c),
        }
    }
}

struct Field {
    field: Vec<Vec<Tile>>,
}

impl Field {
    fn new(strings: Vec<&str>) -> Self {
        let field = strings
            .into_iter()
            .map(|str| str.chars().map(Tile::from_char).collect::<Vec<Tile>>())
            .collect::<Vec<Vec<Tile>>>();

        Field { field }
    }

    fn move_slope(&self, times_r: usize, times_d: usize) -> u32 {
        let mut trees_met = 0;
        let mut index_i = 0;
        let mut index_j = 0;

        while index_i != self.field.len() - 2 {
            index_j = (index_j + times_r) % self.field[index_i].len();
            index_i = (index_i + times_d) % self.field.len();
            if self.field[index_i][index_j] == Tile::Tree {
                trees_met += 1;
            }
        }

        trees_met
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/input3").unwrap();

    let strings = input.split('\n').collect::<Vec<&str>>();

    let f = Field::new(strings);

    let trees_3_1 = f.move_slope(3, 1);
    let trees_1_1 = f.move_slope(1, 1);
    let trees_5_1 = f.move_slope(5, 1);
    let trees_7_1 = f.move_slope(7, 1);
    let trees_1_2 = f.move_slope(1, 2);

    println!("Trees met {}", trees_3_1);
    println!(
        "All trees met {}",
        trees_3_1 * trees_1_1 * trees_5_1 * trees_7_1 * trees_1_2
    );
}

#[derive(Debug, PartialEq)]
enum Seat {
    Occupied,
    Free,
    Floor,
}

// todo: part2
fn neighboors(i: usize, j: usize, max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    let ii = i as i32;
    let jj = j as i32;
    vec![
        (ii + 1, jj),
        (ii - 1, jj),
        (ii, jj + 1),
        (ii, jj - 1),
        (ii + 1, jj + 1),
        (ii + 1, jj - 1),
        (ii - 1, jj + 1),
        (ii - 1, jj - 1),
    ]
    .iter()
    .filter(|(i, j)| *i >= 0 && *i < max_i as i32 && *j >= 0 && *j < max_j as i32)
    .map(|(i, j)| (*i as usize, *j as usize))
    .collect()
}

fn step(field: &Vec<Vec<Seat>>) -> Vec<(usize, usize, Seat)> {
    field
        .iter()
        .enumerate()
        .flat_map(move |(idx, row)| {
            row.iter().enumerate().flat_map(move |(idx2, _el)| {
                let neighboors: Vec<(usize, usize)> = neighboors(idx, idx2, field.len(), row.len());
                let ocn = neighboors
                    .into_iter()
                    .map(|(i, j)| match field[i][j] {
                        Seat::Occupied => 1,
                        _ => 0,
                    })
                    .sum::<u32>();

                match (ocn, &field[idx][idx2]) {
                    (0, Seat::Free) => Some((idx, idx2, Seat::Occupied)),
                    (x, Seat::Occupied) if x >= 4 => Some((idx, idx2, Seat::Free)),
                    (_, Seat::Floor) => None,
                    _ => None,
                }
            })
        })
        .collect()
}

fn update(field: &mut Vec<Vec<Seat>>, updates: Vec<(usize, usize, Seat)>) {
    updates.into_iter().for_each(|(i, j, s)| field[i][j] = s)
}

fn play(mut field: Vec<Vec<Seat>>) -> usize {
    loop {
        let updates = step(&field);
        display(&field);
        if updates.len() == 0 {
            break;
        }
        update(&mut field, updates);
    }

    field
        .into_iter()
        .map(|r| r.into_iter().filter(|s| *s == Seat::Occupied).count())
        .sum()
}

fn display(field: &Vec<Vec<Seat>>) {
    for r in field {
        for s in r {
            match s {
                Seat::Occupied => print!("#"),
                Seat::Free => print!("L"),
                Seat::Floor => print!("."),
            }
        }
        println!()
    }
    println!()
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input11").unwrap();

    let field = inputs
        .split('\n')
        .filter(|str| !str.is_empty())
        .map(|str| {
            str.chars()
                .map(|c| match c {
                    'L' => Seat::Free,
                    '.' => Seat::Floor,
                    _ => panic!("invalid char for seat {}", c),
                })
                .collect::<Vec<Seat>>()
        })
        .collect::<Vec<Vec<Seat>>>();

    let occupied = play(field);

    println!("occupied {}", occupied);
}

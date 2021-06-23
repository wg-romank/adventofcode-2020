use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Seat {
    Occupied,
    Free,
    Floor,
}

// todo: part2
fn directions() -> Vec<fn(i64, i64) -> (i64, i64)> {
    vec![
        |ii, jj| { (ii + 1, jj) },
        |ii, jj| { (ii - 1, jj) },
        |ii, jj| { (ii, jj + 1) },
        |ii, jj| { (ii, jj - 1) },
        |ii, jj| { (ii + 1, jj + 1) },
        |ii, jj| { (ii + 1, jj - 1) },
        |ii, jj| { (ii - 1, jj + 1) },
        |ii, jj| { (ii - 1, jj - 1) },
    ]
}

fn neighboors_pt1(ii: usize, jj: usize, max_i: usize, max_j: usize) -> impl Iterator<Item=(usize, usize)> {
    directions()
        .into_iter()
        .map(move |f| f(ii as i64, jj as i64))
        .filter(move |(i, j)| *i >= 0 && *i < max_i as i64 && *j >= 0 && *j < max_j as i64)
        .map(|(i, j)| (i as usize, j as usize))
}

fn step<I: Iterator<Item=(usize, usize)>>(field: &Vec<Vec<Seat>>, neighboors_fn: fn(usize, usize, usize, usize) -> I) -> Vec<(usize, usize, Seat)> {
    field
        .iter()
        .enumerate()
        .flat_map(move |(idx, row)| {
            row.iter().enumerate().flat_map(move |(idx2, _el)| {
                match field[idx][idx2] {
                    Seat::Floor => None,
                    Seat::Free => {
                        // check if 0 neighboors
                        neighboors_fn(idx, idx2, field.len(), row.len())
                            .into_iter()
                            .fold_while(Some((idx, idx2, Seat::Occupied)), |acc, (i, j)|
                                if field[i][j] == Seat::Occupied { Done(None) } else { Continue(acc) }
                            ).into_inner()
                        },
                    Seat::Occupied => {
                        // check if at least 4 neighboors
                        neighboors_fn(idx, idx2, field.len(), row.len())
                            .into_iter()
                            .fold_while((0, None), |(acc, _), (i, j)| {
                                let new_acc = if field[i][j] == Seat::Occupied { acc + 1 } else { acc };
                                if new_acc >= 4 { Done((0, Some((idx, idx2, Seat::Free)))) } else { Continue((new_acc, None))}
                            }).into_inner().1
                    }
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
        let updates = step(&field, neighboors_pt1);
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

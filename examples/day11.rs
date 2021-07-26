#[derive(Debug, PartialEq, Clone)]
enum Seat {
    Occupied,
    Free,
    Floor,
}

fn directions() -> Vec<fn(i64, i64) -> (i64, i64)> {
    vec![
        |ii, jj| (ii + 1, jj),
        |ii, jj| (ii - 1, jj),
        |ii, jj| (ii, jj + 1),
        |ii, jj| (ii, jj - 1),
        |ii, jj| (ii + 1, jj + 1),
        |ii, jj| (ii + 1, jj - 1),
        |ii, jj| (ii - 1, jj + 1),
        |ii, jj| (ii - 1, jj - 1),
    ]
}

fn neighboors_pt1(
    ii: usize,
    jj: usize,
    max_i: usize,
    max_j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    directions()
        .into_iter()
        .map(move |f| f(ii as i64, jj as i64))
        .filter(move |(i, j)| *i >= 0 && *i < max_i as i64 && *j >= 0 && *j < max_j as i64)
        .map(|(i, j)| (i as usize, j as usize))
}

fn neighboors_pt2(
    ii: usize,
    jj: usize,
    max_i: usize,
    max_j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let initials = neighboors_pt1(ii, jj, max_i, max_j).collect::<Vec<(usize, usize)>>();

    directions()
        .into_iter()
        .cycle() // todo: fix endless loop here
        .scan(
            (0, initials),
            move |(idx, acc), f: fn(i64, i64) -> (i64, i64)| {
                if acc.len() > 0 {
                    let (i, j) = acc[*idx];
                    let (ii, jj) = f(i as i64, j as i64);
                    if ii >= 0 && ii < max_i as i64 && jj >= 0 && jj < max_j as i64 {
                        acc[*idx] = (ii as usize, jj as usize);
                    } else {
                        acc.remove(*idx);
                    }
                    *idx = (*idx + 1) % acc.len();
                    Some((i, j))
                } else {
                    None
                }
            },
        )
}

fn step<I: Iterator<Item = (usize, usize)>>(
    field: &mut Vec<Vec<Seat>>,
    neighboors_fn: fn(usize, usize, usize, usize) -> I,
    tol: usize,
) -> usize {
    // todo: less allocations?
    let field_snapshot = field.clone();
    let height = field.len();
    let width = field[0].len();

    let mut updates = 0;

    for row in 0..height {
        for col in 0..width {
            match field_snapshot[row][col] {
                Seat::Floor => (),
                Seat::Free => {
                    let mut no_occupied_neighboors = true;

                    for (i, j) in neighboors_fn(row, col, height, width) {
                        if field_snapshot[i][j] == Seat::Occupied {
                            no_occupied_neighboors = false;
                            break;
                        }
                    }

                    if no_occupied_neighboors {
                        field[row][col] = Seat::Occupied;
                        updates += 1;
                    }
                }
                Seat::Occupied => {
                    let mut occupied_neighboors = 0;

                    for (i, j) in neighboors_fn(row, col, height, width) {
                        if field_snapshot[i][j] == Seat::Occupied {
                            occupied_neighboors += 1;
                            if occupied_neighboors >= tol {
                                break;
                            }
                        }
                    }

                    if occupied_neighboors >= tol {
                        field[row][col] = Seat::Free;
                        updates += 1
                    };
                }
            }
        }
    }

    updates
}

fn play<I: Iterator<Item = (usize, usize)>>(
    mut field: Vec<Vec<Seat>>,
    neighboors_fn: fn(usize, usize, usize, usize) -> I,
    tol: usize,
) -> usize {
    loop {
        let updates = step(&mut field, neighboors_fn, tol);

        // display(&field);

        if updates == 0 {
            break;
        }
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
    let inputs = std::fs::read_to_string("inputs/input11").expect("no input file");

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

    let occupied = play(field.clone(), neighboors_pt1, 4);
    println!("occupied {}", occupied);

    // let occupied2 = play(field, neighboors_pt2, 5);
    // println!("occupied pt2 {}", occupied2);
}

#[test]
fn test_neighboors_pt2() {
    neighboors_pt2(3, 3, 6, 6).for_each(|f| println!("{:#?}", f));
}

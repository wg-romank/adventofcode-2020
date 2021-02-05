// todo: refactor a bit

fn part2(input: &str) -> (i32, i32, i32, i32) {
    input
        .split("\n")
        .filter(|str| !str.is_empty())
        .fold((0, 0, 10, 1), |(x, y, wx, wy), line| {
            let (c, rest): (&str, &str) = line.split_at(1);
            let value = rest.parse::<i32>().unwrap();
            match c {
                "N" => (x, y, wx, wy + value),
                "S" => (x, y, wx, wy - value),
                "E" => (x, y, wx + value, wy),
                "W" => (x, y, wx - value, wy),
                "L" | "R" => {
                    let rads = (if c == "R" { - value } else { value } as f64).to_radians();
                    let rcos = rads.cos() as i32;
                    let rsin = rads.sin() as i32;
                    (x, y, wx * rcos - wy * rsin, wx * rsin + wy * rcos)
                },
                "F" => {
                    (x + value * wx, y + value * wy, wx, wy)
                },
                _ => panic!("unknown command {}", c),
            }
        })
}

fn main() {
    let input = std::fs::read_to_string("inputs/input12").unwrap();

    let (x, y, _) = input
        .split("\n")
        .filter(|str| !str.is_empty())
        .fold((0, 0, (1_f64, 0_f64)), |(x, y, dir), line| {
            let (c, rest): (&str, &str) = line.split_at(1);
            let value = rest.parse::<i32>().unwrap();
            match c {
                "N" => (x, y + value, dir),
                "S" => (x, y - value, dir),
                "E" => (x + value, y, dir),
                "W" => (x - value, y, dir),
                "L" | "R" => {
                    let current_dir = dir.1.atan2(dir.0).to_degrees();
                    let new_dir = (current_dir + if c == "R" { - value } else { value } as f64).to_radians();
                    (x, y, (new_dir.cos(), new_dir.sin()))
                },
                "F" => {
                    (x + value * (dir.0 as i32), y + value * (dir.1 as i32), dir)
                },
                _ => panic!("unknown command {}", c),
            }
        });

    println!("coordinates {}", x.abs() + y.abs());

    let (x1, y1, _, _) = part2(&input);

    println!("coordinates part 2 {}", x1.abs() + y1.abs());
}

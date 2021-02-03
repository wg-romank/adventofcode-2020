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
}

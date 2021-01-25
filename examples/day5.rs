fn binary_search(row: &str, low: u16, high: u16) -> u16 {
    row.chars().fold((low, high), |(low, high), c| {
        match c {
            'F' | 'L' => (low, (low + high) / 2),
            'B' | 'R' => ((low + high) / 2 + 1, high),
            _ => panic!("Malformed row {}", row),
        }
    }).0
}

fn seat_id(boarding: &str) -> u16 {
    let (row, seat) = boarding.split_at(7);

    let row = binary_search(row, 0, 127);
    let col = binary_search(seat, 0, 7);

    row * 8 + col
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input5").unwrap();

    let mut ids = inputs
        .split('\n')
        .filter(|&str| { !str.is_empty() })
        .map(|str| { seat_id(str) }).collect::<Vec<u16>>();
    ids.sort();

    let max_id = ids.last();
    println!("max_id {:#?}", max_id);

    let my_id = ids[3..].into_iter().fold(([ids[0], ids[1], ids[3]], None), |(acc, _), &next| {
       if acc[0] != acc[1] + 1 {
           println!("{:#?}", acc);
           (acc, Some(acc[1]))
       } else if acc[1] != acc[2] + 1 {
           println!("{:#?}", acc);
           (acc, Some(acc[2]))
       } else {
           ([acc[1], acc[2], next], None)
       }
    }).1;

    println!("my id {:#?}", my_id);
}

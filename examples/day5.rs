use itertools::sorted;

fn binary_search(row: &str, low: u16, high: u16) -> Option<u16> {
    row.chars()
        .fold(Some((low, high)), |acc, c| match acc {
            Some((low, high)) => match c {
                'F' | 'L' => Some((low, (low + high) / 2)),
                'B' | 'R' => Some(((low + high) / 2 + 1, high)),
                _ => None,
            },
            None => None,
        })
        .map(|(result, _)| result)
}

fn seat_id(boarding: &str) -> Option<u16> {
    if boarding.len() > 7 {
        let (row, seat) = boarding.split_at(7);

        let row = binary_search(row, 0, 127)?;
        let col = binary_search(seat, 0, 7)?;

        Some(row * 8 + col)
    } else {
        None
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input5").expect("no input file");

    let ids = sorted(inputs.split('\n').flat_map(seat_id)).collect::<Vec<u16>>();

    let first_id_option = ids.first().map(|v| *v);
    let last_id_option = ids.last().map(|v| *v);

    match (first_id_option, last_id_option) {
        (Some(first), Some(last)) => {
            println!("max id {:?}", last);
            let missing = (first..last)
                .zip(ids)
                .skip_while(|(l, r)| l == r)
                .map(|(l, _)| l)
                .next();

            println!("my id {:?}", missing);
        }
        _ => (),
    }
}

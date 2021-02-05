use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/input13").unwrap();
    let mut ii = input.split('\n');

    let ts = ii.next().map(|str| str.parse::<u64>().ok()).flatten().unwrap();
    let busses = ii.next().map(
        |str| str
            .split(',')
            .enumerate()
            .filter(|(_, str)| !str.starts_with("x"))
            .flat_map(|(idx, str): (usize, &str)| {
                str.parse::<u64>().ok().map(|b| (idx, b))
            })
            .collect::<Vec<(usize, u64)>>()
    ).unwrap();

    let result_p1 = busses
        .iter()
        .map(|&(_, b)| (b, b - ts % b))
        .sorted_by(|a, b| a.1.cmp(&b.1)).next();
    println!("result {:#?}", result_p1);

    let result_p2 = busses
        .iter()
        .map(|&(idx, b)| ((b - (idx as u64 % b)) % b, b))
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .fold((0, 1), |(acc, v): (u64, u64), (ai, ni): (u64, u64)| {
            let mut i = 1;
            while (acc + i * v) % ni != ai {
                i += 1;
            };
            (acc + i * v, v * ni)
        });
    println!("result2 {:#?}", result_p2);
}

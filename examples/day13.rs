use itertools::Itertools;
use itertools::FoldWhile::{Done, Continue};

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
            // a bit obscure but cps indeed
            let new_acc = (1..).fold_while(acc, |ac, _|
                if ac % ni == ai { Done(ac) }
                else { Continue(ac + v) }
            ).into_inner();

            (new_acc, v * ni)
        });
    println!("result2 {:#?}", result_p2);
}

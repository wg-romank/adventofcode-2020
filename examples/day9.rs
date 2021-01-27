use itertools::Itertools;

fn find_invalid(items: Vec<u64>, k: usize) -> Option<u64> {
    items[k..].iter().fold(
        (Vec::from(&items[..k][..]), None), |(mut prev, acc), &next| {
            let x = prev
                .iter()
                .combinations(2)
                .any(|c| c.iter().map(|&&c| c).sum::<u64>() == next);
            if x {
                prev.remove(0);
                prev.push(next);
                (prev, acc)
            } else {
                (prev, acc.or(Some(next)))
            }
        }).1
}

fn main() {
    let input = std::fs::read_to_string("inputs/input9")
        .unwrap()
        .split('\n')
        .filter(|str| !str.is_empty())
        .flat_map(|str| str.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    println!("invalid {:#?}", find_invalid(input, 25));
}

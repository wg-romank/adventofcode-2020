use itertools::Itertools;

fn find_invalid(items: &Vec<u64>, k: usize) -> Option<u64> {
    items[k..]
        .iter()
        .fold(
            (Vec::from(&items[..k][..]), None),
            |(mut prev, acc), &next| {
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
            },
        )
        .1
}

fn find_contigeous(items: &Vec<u64>, num: u64) -> u64 {
    for start in 0..items.len() {
        let mut cum = 0;
        for end in start..items.len() {
            if cum < num {
                cum += items[end];
            } else if cum == num && end > start + 1 {
                let (min_n, max_n) = items[start..end]
                    .iter()
                    .fold((u64::max_value(), 0), |(min_n, max_n), &next| {
                        (min_n.min(next), max_n.max(next))
                    });
                return min_n + max_n;
            } else {
                break;
            }
        }
    }
    panic!("did not find matching set of values")
}

fn main() {
    let input = std::fs::read_to_string("inputs/input9")
        .unwrap()
        .split('\n')
        .filter(|str| !str.is_empty())
        .flat_map(|str| str.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let invalid_number = find_invalid(&input, 25).unwrap();
    println!("invalid {}", invalid_number);

    println!("contigeous {}", find_contigeous(&input, invalid_number));
}

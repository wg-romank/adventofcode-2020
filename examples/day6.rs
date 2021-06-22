use std::collections::HashSet;

fn count_answers(blank: &str) -> u32 {
    blank
        .split('\n')
        .flat_map(|str| str.chars())
        .fold(HashSet::new(), |mut acc, next| {
            acc.insert(next);
            acc
        })
        .len() as u32
}

fn count_answers2(blank: &str) -> u32 {
    blank
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|str| str.chars().collect::<HashSet<char>>())
        .fold(None, |acc, next| match acc {
            None => Some(next),
            // todo: without copied?
            Some(p) => Some(p.intersection(&next).copied().collect()),
        })
        .map(|s| s.len() as u32)
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("inputs/input6").unwrap();

    let total_yes: (u32, u32) = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|blank| (count_answers(blank), count_answers2(blank)))
        .fold((0, 0), |(y1, y2), (n1, n2)| (y1 + n1, y2 + n2));

    println!("total yes {:#?}", total_yes);
}

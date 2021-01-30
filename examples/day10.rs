use itertools::Itertools;
use std::collections::HashSet;

fn can_connect(jolt1: u32, jolt2: u32) -> bool {
    (jolt1 as i32 - jolt2 as i32).abs() <= 3
}

fn count_ways(adapters: &[u32], idx: usize, target_joltage: u32) -> u32 {
    if can_connect(adapters[idx], target_joltage) { 1 }
    else {
        let new_idx = idx + 1;
        adapters[new_idx..]
            .iter()
            .enumerate()
            .take_while(|(_, &ad)| can_connect(ad, adapters[idx]))
            .map(|(offset, _)| count_ways(&adapters, new_idx + offset, target_joltage))
            .fold(0, |acc, v| acc + v)
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input10").unwrap();

    let adapters = inputs
        .split('\n')
        .flat_map(|str| str.parse::<u32>().ok())
        .sorted()
        .collect::<Vec<u32>>();

    let target_joltage = adapters
        .iter()
        .fold(u32::min_value(), |acc, &v| if v >= acc { v } else { acc }) + 3;

    let (j1diff, j3diff, _) = adapters
        .iter()
        .fold((0, 0, 0), |(j1diff, j3diff, last), &next|
            match next - last {
                1 => (j1diff + 1, j3diff, next),
                3 => (j1diff, j3diff + 1, next),
                _ => (j1diff, j3diff, next),
            });

    println!("jolts {:#?}", j1diff * (j3diff + 1));

    println!("target joltage {}", target_joltage);

    println!("configurations {}", count_ways(&[&[0], adapters.as_slice()].concat(), 0, target_joltage))
}

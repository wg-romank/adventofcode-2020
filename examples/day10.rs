use itertools::Itertools;
use std::collections::HashMap;

fn can_connect(jolt1: u32, jolt2: u32) -> bool {
    (jolt1 as i32 - jolt2 as i32).abs() <= 3
}

fn cw(adapters: Vec<u32>, target_joltage: u32) -> u64 {
    let m: HashMap<u32, u64> = adapters.iter().rev().fold(HashMap::new(), |mut m, &jolt| {
        let cc = u64::max(
            1,
            (jolt..target_joltage)
                .filter(|&j| can_connect(j, jolt)) // todo: takewhile?
                .rev()
                .flat_map(|j| m.get(&j))
                .sum::<u64>(),
        );
        m.insert(jolt, u64::max(1, cc));
        m
    });
    m[&0]
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input10").expect("missing input file");

    let mut adapters = inputs
        .split('\n')
        .flat_map(|str| str.parse::<u32>().ok())
        .sorted()
        .collect::<Vec<u32>>();
    adapters.insert(0, 0);

    let target_joltage = adapters
        .iter()
        .max()
        .map(|&j| j + 3)
        .expect("empty adapters list?");

    let (j1diff, j3diff, _) = adapters
        .iter()
        .fold((0, 0, 0), |(j1diff, j3diff, last), &next| {
            match next - last {
                1 => (j1diff + 1, j3diff, next),
                3 => (j1diff, j3diff + 1, next),
                _ => (j1diff, j3diff, next),
            }
        });

    println!("jolts {:#?}", j1diff * (j3diff + 1));

    println!("target joltage {}", target_joltage);

    println!("configurations {}", cw(adapters, target_joltage));
}

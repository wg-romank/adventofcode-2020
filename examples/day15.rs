use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn turn(mut numbers: HashMap<usize, usize>, mut last_spoken: usize, tid_stop: usize) -> usize {
    let mut tid = numbers.len() + 1;

    loop {
        let next_spoken = match numbers.entry(last_spoken) {
            Entry::Occupied(a) => tid - a.get(),
            Entry::Vacant(_) => 0,
        };
        numbers.insert(last_spoken, tid);

        if tid == tid_stop {
            break last_spoken;
        };

        tid += 1;
        last_spoken = next_spoken;
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input15").unwrap();

    let initial_numbers = inputs
        .split(',')
        .flat_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    let last_spoken = *initial_numbers.last().expect("no numbers");

    let map = initial_numbers[..initial_numbers.len() - 1]
        .iter()
        .enumerate()
        .map(|(idx, n)| (*n, idx + 1))
        .collect::<HashMap<usize, usize>>();

    println!(
        "2020th spoken {}",
        turn(map.clone(), last_spoken, 2020)
    );
    println!(
        "30000000th spoken {}",
        turn(map, last_spoken, 30000000)
    );
}

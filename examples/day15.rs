use std::collections::HashMap;

fn turn(tid: usize, numbers: HashMap<usize, usize>, tid_stop: usize, last_spoken: usize) -> usize {
    let mut tid = tid;
    let mut last_spoken = last_spoken;
    let mut numbers = numbers;

    loop {
        let next_spoken = if let Some(num) = numbers.to_owned().get(&last_spoken) {
            numbers.insert(last_spoken, tid);
            tid - num
        } else {
            numbers.insert(last_spoken, tid);
            0
        };

        println!("{} next spoken {}", tid, next_spoken);

        if tid == tid_stop {
            break last_spoken;
        };

        tid = tid + 1;
        last_spoken = next_spoken;
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input15").unwrap();

    let initial_numbers = inputs
        .split(',')
        .flat_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    let last_spoken = *initial_numbers.last().ok_or("no numbers").unwrap();

    println!("{:?}", initial_numbers);
    println!("last spoken {:?}", last_spoken);

    let map = initial_numbers[..initial_numbers.len() - 1]
        .into_iter()
        .enumerate()
        .map(|(idx, n)| (*n, idx + 1))
        .collect::<HashMap<usize, usize>>();

    // println!(
    //     "2020th spoken {}",
    //     turn(map.len() + 1, map, 2020, last_spoken)
    // );
    println!(
        "30000000th spoken {}",
        turn(map.len() + 1, map, 30000000, last_spoken)
    );
}

use std::str::FromStr;

// Dumb but working
fn main() {
    let inputs = std::fs::read_to_string("inputs/input1")
        .unwrap()
        .split('\n')
        .flat_map(FromStr::from_str)
        .collect::<Vec<u32>>();

    // todo: cps?
    for &i in inputs.iter() {
        for &j in inputs.iter() {
            if i + j == 2020 {
                println!("i = {} j = {}, i * j = {}", i, j, i * j);
                return;
            }
        }
    }

    for &i in inputs.iter() {
        for &j in inputs.iter() {
            for &k in inputs.iter() {
                if i + j + k == 2020 {
                    println!("i = {} j = {} k = {}, i * j * k = {}", i, j, k, i * j * k);
                    return;
                }
            }
        }
    }
}

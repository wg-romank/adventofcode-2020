use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use mod_exp::mod_exp;

fn baby_step_giant_step(beta: u64, a: u64, n: u64) -> u64 {
    // Reference https://en.wikipedia.org/wiki/Baby-step_giant-step
    let m = (n as f64).sqrt().ceil() as u64; // modulus is group order

    let lookup = (0..m)
        .map(|j| (mod_exp(a, j, n), j))
        .collect::<HashMap<u64, u64>>();

    // a ^ m (n - 2) = a ^ mn * a ^ -2m = a ^ m * a ^ -2m = a ^ -m
    // a ^ n == a (mod n), Fermat little theorem
    // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
    let a_m = mod_exp(a, m * (n - 2), n);

    (0..m)
        .fold_while(beta, |gamma, i| {
            if let Some(j) = lookup.get(&gamma) {
                Done((i * m + j) % n)
            } else {
                Continue((gamma * a_m) % n)
            }
        })
        .into_inner()
}

fn main() {
    let subj = 7;
    let n = 20201227;

    let door_pubkey = 8252394;
    let card_pubkey = 6269621;

    let door_loop = baby_step_giant_step(door_pubkey, subj, n);

    println!("door loop {}", door_loop);

    assert_eq!(door_pubkey, mod_exp(subj, door_loop, n));

    println!("encryption key {}", mod_exp(card_pubkey, door_loop, n));
}

#[test]
fn test_find_loop_size() {
    let loop_size = baby_step_giant_step(5764801, 7, 20201227);
    assert_eq!(8, loop_size);
}

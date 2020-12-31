use std::collections::HashMap;

use mod_exp::mod_exp;

fn loop_steps(loop_size: u64, input: u64) -> u64 {
    let mut result = 1;
    for _ in 0..loop_size {
        result *= input;
        result %= 20201227;
    }

    result
}

fn baby_step_giant_step(beta: u64, a: u64) -> u64 {
    // Reference https://en.wikipedia.org/wiki/Baby-step_giant-step
    let n = 20201227;
    let m = (20201227 as f32).sqrt().ceil() as u64; // modulus is group order

    let mut lookup = HashMap::new();

    for j in 0..m {
        lookup.insert(mod_exp(a, j, n), j);
    }

    // a ^ m (n - 2) = a ^ mn * a ^ -2m = a ^ m * a ^ -2m = a ^ -m
    // a ^ n == a (mod n), Fermat little theorem
    // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
    let a_m = mod_exp(a, m * (n - 2), n);

    let mut gamma = beta; // member of the group
    for i in 0..m {
        if let Some(j) = lookup.get(&gamma) {
            return (i * m + j) % n
        }

        gamma = (gamma * a_m) % n;
    }

    panic!("Not found exponent for {}", beta);
}


fn main() {
    let subj = 7;
    let door_pubkey = 8252394;
    let card_pubkey = 6269621;

    let door_loop = baby_step_giant_step(door_pubkey, subj);

    println!("door loop {}", door_loop);

    assert_eq!(door_pubkey, loop_steps(door_loop, subj));

    println!("encryption key {}", loop_steps(door_loop, card_pubkey))
}

#[test]
fn test_loop_steps() {
    assert_eq!(5764801, loop_steps(8, 7));
}

#[test]
fn test_find_loop_size() {
    let loop_size = baby_step_giant_step(5764801, 7);
    assert_eq!(8, loop_size);
}

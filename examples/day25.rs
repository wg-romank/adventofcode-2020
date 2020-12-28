fn loop_steps(loop_size: u8, subj: u32) -> u32 {
    let mut result = 1;
    for _ in 0..loop_size {
        result *= subj;
        result %= 20201227;
    }

    result
}

fn find_loop_size(pubkey1: u32, pubkey2: u32) -> (u8, u8, u32) {
    let mut subj = 2;
    let mut loop1 = 1;
    let mut loop2 = 2;

    // loop1 != loop2
    // first assuming loop is u8 which might not be the case here

    loop {
        // todo: walk?

        if loop_steps(loop1, subj) == pubkey1 && loop_steps(loop2, subj) == pubkey2 {
            break
        }
    }

    (loop1, loop2, subj)
}


fn main() {
    let door_pubkey = 8252394;
    let card_pubkey = 6269621;

    let (door_loop, card_loop, subj) = find_loop_size(door_pubkey, card_pubkey);

    println!("door loop {} card loop {} subject {}", door_loop, card_loop, subj);

    println!("encryption key {}", loop_steps(door_loop, door_pubkey))
}

#[test]
fn test_loop_steps() {
    assert_eq!(5764801, loop_steps(8, 7));
}

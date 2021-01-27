use std::collections::HashSet;

fn compute_acc(instructions: &Vec<&str>, idx: usize, mut visited: HashSet<usize>, acc: i32) -> i32 {
    if visited.contains(&idx) {
        acc
    } else {
        visited.insert(idx);
        let (next_idx, next_acc) = match instructions[idx] {
            x if x.starts_with("nop") => (idx + 1, acc),
            x if x.starts_with("acc") => {
                let v = x.split(" ").last().map(|i| i.parse::<i32>().ok()).flatten().unwrap();
                (idx + 1, acc + v)
            },
            x if x.starts_with("jmp") => {
                let v = x.split(" ").last().map(|i| i.parse::<i32>().ok()).flatten().unwrap();
                ((idx as i32 + v) as usize, acc)
            },
            other => panic!("unknown instruction {}", other),
        };

        compute_acc(instructions, next_idx, visited, next_acc)
    }
}


fn main() {
    let input = std::fs::read_to_string("inputs/input8").unwrap();

    let instructions = input.split('\n').collect();

    println!("acc value {}", compute_acc(&instructions, 0, HashSet::new(), 0));
}

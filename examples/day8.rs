use std::collections::HashSet;

fn nop(_x: &str, idx: usize, acc: i32) -> (usize, i32) {
    (idx + 1, acc)
}

fn acc(x: &str, idx: usize, acc: i32) -> (usize, i32) {
    let v = x.split(" ").last().map(|i| i.parse::<i32>().ok()).flatten().unwrap();
    (idx + 1, acc + v)
}

fn jmp(x: &str, idx: usize, acc: i32) -> (usize, i32) {
    let v = x.split(" ").last().map(|i| i.parse::<i32>().ok()).flatten().unwrap();
    ((idx as i32 + v) as usize, acc)
}

fn compute_acc(instructions: &Vec<&str>, idx: usize, swap_idx: usize, mut visited: HashSet<usize>, ac: i32) -> Result<i32, i32> {
    if visited.contains(&idx) {
        Err(ac)
    } else if idx > instructions.len() - 2 {
        Ok(ac)
    } else {
        visited.insert(idx);
        let (next_idx, next_acc) = match instructions[idx] {
            // todo: oneliner?
            x if x.starts_with("nop") => if idx != swap_idx { nop(x, idx, ac) } else { jmp(x, idx, ac) },
            x if x.starts_with("acc") => acc(x, idx, ac),
            x if x.starts_with("jmp") => if idx != swap_idx { jmp(x, idx, ac) } else { nop(x, idx, ac) },
            other => panic!("unknown instruction {}", other),
        };

        compute_acc(instructions, next_idx, swap_idx, visited, next_acc)
    }
}

fn fix_corrupted_instruction(instructions: &Vec<&str>) -> Option<i32> {
    instructions.iter().enumerate().fold(None, |acc, (idx, &v)| {
        if v.starts_with("nop") || v.starts_with("jmp") {
            match compute_acc(instructions, 0, idx, HashSet::new(), 0) {
                Ok(v) => Some(v),
                Err(_) => acc
            }
        } else { acc }
    })
}


fn main() {
    let input = std::fs::read_to_string("inputs/input8").unwrap();

    let instructions = input.split('\n').collect();

    println!("acc value {:#?}", compute_acc(&instructions, 0, usize::max_value(), HashSet::new(), 0));
    println!("acc value fixed {:#?}", fix_corrupted_instruction(&instructions));
}

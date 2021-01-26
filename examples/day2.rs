use itertools::Itertools;

fn policy_maker(policy: &str, str: &str) -> Option<(bool, bool)> {
    // TODO: Scala-style string pattern matching?
    // checkout rustm?
    let (times, letter) = policy.split(' ').next_tuple()?;
    let letter_char = letter.chars().nth(0)?;
    let (times_low, times_high) = times.split('-').next_tuple()?;
    let times_low_i = times_low.parse::<usize>().ok()?;
    let times_high_i = times_high.parse::<usize>().ok()?;

    let count = str.chars().fold(0, |acc, next|
        if next == letter_char { acc + 1 } else { acc }
    );

    let rule1 = times_low_i <= count && count <= times_high_i;
    let rule2 =
        (str.chars().nth(times_low_i)? == letter_char) ^
            (str.chars().nth(times_high_i)? == letter_char);

    Some((rule1, rule2))
}


fn is_valid_password(password_policy: &str) -> Option<(bool, bool)> {
    let (policy, password) = password_policy
        .split(':')
        .next_tuple()?;

    policy_maker(policy, password)
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input2")
        .unwrap();

    let parsed_inputs = inputs
        .split('\n')
        .collect::<Vec<&str>>();

    let (count1, count2) = parsed_inputs
        .into_iter()
        .flat_map(is_valid_password)
        .fold((0, 0), |(acc_a, acc_b), (a, b)| match (a, b) {
            (true, true) => (acc_a + 1, acc_b + 1),
            (false, true) => (acc_a, acc_b + 1),
            (true, false) => (acc_a + 1, acc_b),
            (false, false) => (acc_a, acc_b)
        });

    println!("count {} count2 {}", count1, count2);
}

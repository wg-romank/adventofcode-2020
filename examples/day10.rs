use itertools::Itertools;

fn main() {
    let inputs = std::fs::read_to_string("inputs/input10").unwrap();

    let (j1diff, j3diff, _) = inputs
        .split('\n')
        .flat_map(|str| str.parse::<u32>().ok())
        .sorted()
        .fold((0, 0, 0), |(j1diff, j3diff, last), next|
            match next - last {
                1 => (j1diff + 1, j3diff, next),
                3 => (j1diff, j3diff + 1, next),
                _ => (j1diff, j3diff, next),
        });

    println!("jolts {:#?}", j1diff * (j3diff + 1));
}

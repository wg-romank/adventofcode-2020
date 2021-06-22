use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Memory {
    mem: HashMap<u64, u64>,
    mask_0: u64,
    mask_1: u64,
}

impl Memory {
    const PAD: u64 = 0b1111111111111111111111111110000000000000000000000000000000000000;

    fn new() -> Self {
        Memory {
            mem: HashMap::new(),
            mask_0: 1,
            mask_1: 0,
        }
    }

    fn step(self, command: &str) -> Self {
        match command {
            x if x.starts_with("mask = ") => {
                let (mask_0, mask_1) = x.trim_start_matches("mask = ").chars().fold(
                    (1_u64, 1_u64),
                    |(acc0, acc1), c| match c {
                        '1' => (acc0 << 1 | 0b1, acc1 << 1 | 0b0),
                        '0' => (acc0 << 1 | 0b0, acc1 << 1 | 0b1),
                        'X' => (acc0 << 1 | 0b1, acc1 << 1 | 0b1),
                        _ => (acc0, acc1),
                    },
                );
                Memory {
                    mask_0,
                    mask_1,
                    ..self
                }
            }
            x if x.starts_with("mem") => {
                let (l, r) = x.split("=").map(|str| str.trim()).next_tuple().unwrap();
                let address = l
                    .trim_start_matches("mem[")
                    .trim_end_matches("]")
                    .parse::<u64>()
                    .unwrap();
                let value = r.parse::<u64>().unwrap();

                let mut mem = self.mem.to_owned();
                let modified = value & self.mask_0 | !self.mask_1 ^ Memory::PAD;
                mem.insert(address, modified);

                Memory { mem, ..self }
            }
            _ => panic!("unknown command {}", command),
        }
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input14").unwrap();

    let mem = inputs
        .split('\n')
        .filter(|c| !c.is_empty())
        .fold(Memory::new(), |m, c| m.step(c));

    println!("new mem {:#?}", mem);

    let non_zero_cells = mem.mem.iter().map(|(_, &v)| v).sum::<u64>();

    println!("sum {}", non_zero_cells);
}

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
                        '1' => (acc0 << 1 | 0b1, acc1 << 1),
                        '0' => (acc0 << 1, acc1 << 1 | 0b1),
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
                x.split('=').map(|str| str.trim()).next_tuple().iter().fold(
                    self,
                    |mut mem, (l, r)| {
                        let address = l
                            .trim_start_matches("mem[")
                            .trim_end_matches(']')
                            .parse::<u64>()
                            .unwrap();
                        let value = r.parse::<u64>().unwrap();

                        let modified = value & mem.mask_0 | !mem.mask_1 ^ Memory::PAD;
                        mem.mem.insert(address, modified);
                        mem
                    },
                )
            }
            _ => self,
        }
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input14").expect("no input file");

    let mem = inputs.split('\n').fold(Memory::new(), |m, c| m.step(c));

    println!("new mem {:#?}", mem);

    let non_zero_cells = mem.mem.iter().map(|(_, &v)| v).sum::<u64>();

    println!("sum {}", non_zero_cells);
}

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Sub;

#[derive(Clone)]
enum Command {
    Mask(String),
    Mem(u64, u64),
}

impl Command {
    fn from_str(cmd: &str) -> Option<Self> {
        match cmd {
            x if x.starts_with("mask = ") => {
                Some(Command::Mask(x.trim_start_matches("mask = ").to_owned()))
            }
            x if x.starts_with("mem") => {
                x.split('=')
                    .map(|str| str.trim())
                    .next_tuple()
                    .map(|(l, r)| {
                        let address = l
                            .trim_start_matches("mem[")
                            .trim_end_matches(']')
                            .parse::<u64>()
                            .unwrap();
                        let value = r.parse::<u64>().unwrap();

                        Command::Mem(address, value)
                    })
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct MemoryV1 {
    mem: HashMap<u64, u64>,
    mask_0: u64,
    mask_1: u64,
}

impl MemoryV1 {
    fn new() -> Self {
        MemoryV1 {
            mem: HashMap::new(),
            mask_0: 1,
            mask_1: 0,
        }
    }
}

struct MemoryV2 {
    mem: BTreeMap<FloatingAddress, u64>,
    mask: String,
}

impl MemoryV2 {
    fn new() -> Self {
        MemoryV2 {
            mem: BTreeMap::new(),
            mask: String::from("000000000000000000000000000000000000"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct FloatingAddress(String);

impl Ord for FloatingAddress {
    fn cmp(&self, other: &Self) -> Ordering {
        self.low()
            .cmp(&other.low())
            .then(self.high().cmp(&other.high()))
    }
}

impl FloatingAddress {
    fn new(addr: u64, mask: &str) -> Self {
        FloatingAddress(
            format!("{:b}", addr)
                .chars()
                .rev()
                .zip(mask.chars().rev())
                .map(|(l, r)| match (l, r) {
                    (_, '1') => '1',
                    (_, 'X') => 'X',
                    (a, _) => a,
                })
                .collect::<String>()
                .chars()
                .rev()
                .collect(),
            )
    }
    fn high(&self) -> u64 {
        self.0.chars().fold(0, |acc, c| match c {
            'X' | '1' => acc << 1 | 0b1,
            '0' => acc << 1,
            _ => acc,
        })
    }
    fn low(&self) -> u64 {
        self.0.chars().fold(0, |acc, c| match c {
            '1' => acc << 1 | 0b1,
            'X' | '0' => acc << 1,
            _ => acc,
        })
    }

    fn inner_addresses(&self) -> Vec<u64> {
        paths(self.0.chars(), 0)
    }

    fn multiplier(&self) -> u64 {
        (2 as u64).pow(self.0.chars().filter(|&c| c == 'X').count() as u32)
    }
}

fn paths<T : Iterator<Item=char> + Clone>(mut it: T, acc: u64) -> Vec<u64> {
    if let Some(n) = it.next() {
        match n {
            '1' => paths(it, acc << 1 | 0b1),
            '0' => paths(it, acc << 1),
            'X' => {
                // todo: figure out oneline concat, this is pain
                let a = paths(it.clone(), acc<< 1 | 0b1);
                let mut b = paths(it, acc << 1);
                b.extend(&a);
                b
            },
            _ => vec![acc],
        }
    } else {
        vec![acc]
    }
}

impl Sub for FloatingAddress {
    type Output = FloatingAddress;
    fn sub(self, rhs: FloatingAddress) -> FloatingAddress { todo!() }
}

fn step_v1(mut mem: MemoryV1, command: Command) -> MemoryV1 {
    // our values are 36-bits thus padding
    const PAD: u64 = 0b1111111111111111111111111110000000000000000000000000000000000000;

    match command {
        Command::Mask(mask) => {
            let (mask_0, mask_1) = mask
                .chars()
                .fold((1_u64, 1_u64), |(acc0, acc1), c| match c {
                    '1' => (acc0 << 1 | 0b1, acc1 << 1),
                    '0' => (acc0 << 1, acc1 << 1 | 0b1),
                    'X' => (acc0 << 1 | 0b1, acc1 << 1 | 0b1),
                    _ => (acc0, acc1),
                });
            MemoryV1 {
                mask_0,
                mask_1,
                ..mem
            }
        }
        Command::Mem(address, value) => {
            let modified = value & mem.mask_0 | !mem.mask_1 ^ PAD;
            mem.mem.insert(address, modified);
            mem
        }
    }
}

fn step_v2(mut mem: MemoryV2, command: Command) -> MemoryV2 {
    match command {
        Command::Mask(mask) => MemoryV2 { mask, ..mem },
        Command::Mem(address, value) => {
            let new_address = FloatingAddress::new(address, &mem.mask);
            mem.mem.insert(new_address, value);
            mem
        }
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input14").expect("no input file");
    let commands = inputs.split('\n').flat_map(Command::from_str);

    let mem = commands.clone().fold(MemoryV1::new(), |m, c| step_v1(m, c));

    let non_zero_cells = mem.mem.iter().map(|(_, &v)| v).sum::<u64>();

    println!("sum {}", non_zero_cells);

    let mem = commands.fold(MemoryV2::new(), |m, c| step_v2(m, c));

    let non_zero_cells_pt2 = mem.mem.iter().map(|(a, &v)| a.multiplier() * v).sum::<u64>();

    println!("sum pt2 {}", non_zero_cells_pt2);
}

#[test]
fn test_memory_address_decoder() {
    let addr = FloatingAddress::new(26, "00000000000000000000000000000000X0XX");

    assert_eq!(addr.high(), 27);
    assert_eq!(addr.low(), 16);

    assert_eq!(addr.inner_addresses(), vec![16, 17, 18, 19, 24, 25, 26, 27]);
}

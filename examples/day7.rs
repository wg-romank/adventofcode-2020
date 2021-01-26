use itertools::Itertools;

#[derive(Debug)]
struct CanContain<'a> {
    color: &'a str,
    amount: u8,
}

impl<'a> CanContain<'a> {
    fn from_str(str: &'a str) -> Option<Self> {
        // todo: fix issue taking 1st word of color only
        let (amount_raw, color_raw): (&str, &str) = str.splitn(2, ' ').next_tuple()?;
        let amount = amount_raw.parse::<u8>().ok()?;
        let color = color_raw.trim_end_matches(" bags.");

        Some(CanContain { color, amount })
    }
}

#[derive(Debug)]
struct Rule<'a> {
    color: &'a str,
    can_contain: Vec<CanContain<'a>>,
}

impl<'a> Rule<'a> {
    fn from_str(str: &'a str) -> Option<Self> {
        let (color, contain) = str
            .split("bags contain")
            .map(|str| str.trim())
            .next_tuple()?;

        let can_contain = contain
            .split("bags,")
            .flat_map(CanContain::from_str)
            .collect::<Vec<CanContain<'a>>>();

        Some(Rule { color, can_contain })
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input7").unwrap();

    let rules = inputs
        .split('\n')
        .filter(|str| !str.is_empty())
        .flat_map(Rule::from_str)
        .collect::<Vec<Rule>>();

    println!("rules {:#?}", rules);
}

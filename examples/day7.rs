use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct CanContain<'a> {
    color: &'a str,
    amount: u8,
}

impl<'a> CanContain<'a> {
    fn from_str(str: &'a str) -> Option<Self> {
        let (amount_raw, color_raw): (&str, &str) = str.splitn(2, ' ').next_tuple()?;
        let amount = amount_raw.parse::<u8>().ok()?;
        let color = color_raw
            .trim_end_matches(" bags")
            .trim_end_matches(" bag");

        Some(CanContain { color, amount })
    }
}

#[derive(Debug)]
struct RuleSet<'a> {
    rules: HashMap<&'a str, Vec<CanContain<'a>>>,
}

impl<'a> RuleSet<'a> {
    fn from_str(rules_raw: &'a str) -> Self {
        RuleSet {
            rules: rules_raw
                .split('\n')
                .filter(|str| !str.is_empty())
                .flat_map(|str| {
                    let (color, contain) = str
                        .split("bags contain")
                        .map(|str| str.trim())
                        .next_tuple()?;

                    let can_contain = contain
                        .split(&[',', '.'][..])
                        .filter(|str| !str.is_empty())
                        .flat_map(CanContain::from_str)
                        .collect::<Vec<CanContain>>();

                    Some((color, can_contain))
                }).collect()
        }
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input7").unwrap();

    let rules = RuleSet::from_str(inputs.as_str());

    println!("rules {:#?}", rules);
}

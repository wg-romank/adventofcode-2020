use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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
                .flat_map(
                    |str| {
                        let (color, contain) = str
                            .split("bags contain")
                            .map(|str| str.trim())
                            .next_tuple()?;

                        let can_contain = contain
                            .split(&[',', '.'][..])
                            .map(|str| str.trim())
                            .filter(|str| !str.is_empty())
                            .filter(|str| !str.starts_with("no"))
                            .flat_map(CanContain::from_str)
                            .collect::<Vec<CanContain>>();

                        Some((color, can_contain))
                    })
                .collect()
        }
    }
}

fn find_can_contain<'a>(rule_set: &RuleSet<'a>, color: &str) -> Vec<&'a str> {
    rule_set
        .rules
        .iter()
        .filter(
            |(_, r)|
                r.iter().filter(|&c| c.color == color).count() > 0)
        .map(|(&color, _)| color)
        .collect::<Vec<&str>>()
}

fn find_ways<'a>(rule_set: &'a RuleSet<'a>, color: &str, outer_colors: HashSet<&'a str>) -> HashSet<&'a str> {
    let ways = find_can_contain(rule_set, color);
    if ways.len() == 0 {
        outer_colors
    } else {
        let new_outer_colors: HashSet<&str> = outer_colors
            .iter()
            .chain(ways.iter())
            .map(|&c| c)
            .collect();

        ways
            .iter()
            .map(|&c| find_ways(rule_set, c, new_outer_colors.clone()))
            .fold(HashSet::new(), |acc, n| acc.union(&n).copied().collect())
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input7").unwrap();

    let rules = RuleSet::from_str(inputs.as_str());

    let ways = find_ways(&rules, "shiny gold", HashSet::new());

    println!("ways {:#?}", ways.len());
}

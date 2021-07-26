use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Interval {
    from: usize,
    to: usize,
}

impl Interval {
    fn from_str(raw: &str) -> Option<Interval> {
        let (l, r) = raw.split('-').map(str::trim).next_tuple()?;
        let from = l.parse::<usize>().ok()?;
        let to = r.parse::<usize>().ok()?;

        Some(Interval { from, to })
    }

    fn contains(&self, value: usize) -> bool {
        self.from <= value && value <= self.to
    }
}

trait IntervalRange {
    fn contains(&self, value: usize) -> bool;
}

impl IntervalRange for Vec<Interval> {
    fn contains(&self, value: usize) -> bool {
        self.iter().any(|i| i.contains(value))
    }
}

fn parse_header(header: &str) -> HashMap<&str, Vec<Interval>> {
    header
        .split('\n')
        .flat_map(|line| {
            line.split(':')
                .map(str::trim)
                .next_tuple()
                .map(|(l, r)| (l, parse_range(r)))
        })
        .collect::<HashMap<&str, Vec<Interval>>>()
}

fn parse_range(range: &str) -> Vec<Interval> {
    range
        .split("or")
        .map(str::trim)
        .flat_map(Interval::from_str)
        .collect::<Vec<Interval>>()
}

fn parse_ticket(ticket: &str) -> Vec<usize> {
    ticket
        .split(',')
        .map(str::trim)
        .flat_map(|n| n.parse::<usize>().ok())
        .collect()
}

fn check_rules_pt1<IR: IntervalRange>(rules: HashMap<&str, IR>, tickets: Vec<Vec<usize>>) -> usize {
    tickets.into_iter().fold(0_usize, |error_rate, ticket| {
        error_rate
            + ticket
                .into_iter()
                .filter(|&v| rules.values().all(|rule| !rule.contains(v)))
                .sum::<usize>()
    })
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input16").expect("no input file");

    let sections = inputs.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_header(sections[0]);
    let your_ticket = parse_ticket(sections[1]);
    let tickets = sections[2]
        .split('\n')
        .map(parse_ticket)
        .collect::<Vec<Vec<usize>>>();

    println!(
        "ticket scanning error rate {}",
        check_rules_pt1(rules, tickets)
    );
}

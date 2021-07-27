use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Interval {
    from: usize,
    to: usize,
}

impl Interval {
    fn from_str(raw: &str) -> Option<Self> {
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

trait Rules {
    fn count_error_rate(&self, ticket: &[usize]) -> usize;
    fn is_ticket_valid(&self, ticket: &[usize]) -> bool;
}

impl<'a, IR: IntervalRange> Rules for HashMap<&'a str, IR> {
    fn count_error_rate(&self, ticket: &[usize]) -> usize {
        ticket
            .iter()
            .filter(|&v| self.values().all(|rule| !rule.contains(*v)))
            .sum::<usize>()
    }

    fn is_ticket_valid(&self, ticket: &[usize]) -> bool {
        self.count_error_rate(ticket) == 0
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

fn check_rules_pt1<R: Rules>(rules: &R, tickets: &[Vec<usize>]) -> usize {
    tickets.iter().fold(0_usize, |error_rate, ticket| {
        error_rate + rules.count_error_rate(ticket)
    })
}

fn compute_possible_values_pt2<'a, R, IR>(
    rules: &R,
    tickets: Vec<Vec<usize>>,
) -> HashMap<usize, HashSet<&'a str>>
where
    R: Rules + Clone + IntoIterator<Item = (&'a str, IR)>,
    IR: IntervalRange,
{
    let res: Vec<Vec<usize>> = (0..tickets[1].len()).map(|_| Vec::new()).collect();
    let possible_values: Vec<Vec<usize>> = tickets
        .into_iter()
        .filter(|ticket| rules.is_ticket_valid(&ticket))
        .fold(res, |mut res, ticket| {
            for (idx, value) in ticket.iter().enumerate() {
                res[idx].push(*value);
            }
            res
        });

    possible_values
        .into_iter()
        .map(|values| {
            rules
                .clone()
                .into_iter()
                .filter(|(_, r)| values.iter().all(|v| r.contains(*v)))
                .map(|(k, _)| k)
                .collect()
        })
        .enumerate()
        .collect()
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input16").expect("no input file");

    let sections = inputs.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_header(sections[0]);
    let your_ticket = parse_ticket(sections[1].split('\n').collect::<Vec<&str>>()[1]);
    let tickets = sections[2]
        .split('\n')
        .map(parse_ticket)
        .collect::<Vec<_>>();

    println!(
        "ticket scanning error rate {}",
        check_rules_pt1(&rules, &tickets)
    );

    let mut ir = compute_possible_values_pt2(&rules, tickets);
    let mut indices = vec![];

    while let Some((solo_idx, solo_v)) = ir.iter().find(|(_, v)| v.len() == 1) {
        let tmp = *solo_v.iter().next().unwrap_or(&"");

        if tmp.starts_with("departure") {
            indices.push(*solo_idx);
        }

        for v in ir.values_mut() {
            v.remove(tmp);
        }
    }

    let ans: usize = your_ticket
        .iter()
        .enumerate()
        .filter(|(idx, _)| indices.contains(idx))
        .map(|(_, v)| v)
        .product();

    println!("departure values pt2 {:#?}", ans);
}

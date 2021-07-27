use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Interval {
    from: usize,
    to: usize,
}

impl Interval {
    fn from_usize(value: usize) -> Self {
        Interval {
            from: value,
            to: value,
        }
    }
    fn from_str(raw: &str) -> Option<Self> {
        let (l, r) = raw.split('-').map(str::trim).next_tuple()?;
        let from = l.parse::<usize>().ok()?;
        let to = r.parse::<usize>().ok()?;

        Some(Interval { from, to })
    }

    fn contains(&self, value: usize) -> bool {
        self.from <= value && value <= self.to
    }

    fn try_append(&mut self, value: usize) -> Option<Self> {
        if value - 1 <= self.to {
            self.to = usize::max(self.to, value);
            None
        } else {
            Some(Interval {
                from: value,
                to: value,
            })
        }
    }
}

trait IntervalRange {
    fn from_vec(values: Vec<usize>) -> Self;
    fn contains(&self, value: usize) -> bool;
}

impl IntervalRange for Vec<Interval> {
    fn from_vec(values: Vec<usize>) -> Self {
        let first = values[0].clone();

        let (mut v, i) = values[1..].into_iter().fold(
            (vec![], Interval::from_usize(first)),
            |(mut acc, mut current), &value| {
                if let Some(new_interval) = current.try_append(value) {
                    acc.push(current);
                    (acc, new_interval)
                } else {
                    (acc, current)
                }
            },
        );

        v.push(i);
        v
    }

    fn contains(&self, value: usize) -> bool {
        self.iter().any(|i| i.contains(value))
    }
}

trait Rules {
    fn count_error_rate(&self, ticket: &Vec<usize>) -> usize;
    fn is_ticket_valid(&self, ticket: &Vec<usize>) -> bool;
}

impl<'a, IR: IntervalRange> Rules for HashMap<&'a str, IR> {
    fn count_error_rate(&self, ticket: &Vec<usize>) -> usize {
        ticket
            .iter()
            .filter(|&v| self.values().all(|rule| !rule.contains(*v)))
            .sum::<usize>()
    }

    fn is_ticket_valid(&self, ticket: &Vec<usize>) -> bool {
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
    tickets.into_iter().fold(0_usize, |error_rate, ticket| {
        error_rate + rules.count_error_rate(ticket)
    })
}

fn compute_interval_range_pt2<R: Rules, IR: IntervalRange>(
    rules: &R,
    tickets: Vec<Vec<usize>>,
) -> Vec<IR> {
    let res: Vec<Vec<usize>> = (0..tickets[1].len()).map(|_| Vec::new()).collect();
    tickets
        .into_iter()
        .filter(|ticket| rules.is_ticket_valid(ticket))
        .fold(res, |mut res, ticket| {
            for (idx, value) in ticket.iter().enumerate() {
                match res[idx].binary_search(value) {
                    Ok(_) => (),
                    Err(pos) => res[idx].insert(pos, *value),
                }
            }
            res
        })
        .into_iter()
        .map(IntervalRange::from_vec)
        .collect()
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input16").expect("no input file");

    let sections = inputs.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_header(sections[0]);
    let your_ticket = parse_ticket(sections[1]);
    let tickets = sections[2]
        .split('\n')
        .map(parse_ticket)
        .collect::<Vec<_>>();

    println!(
        "ticket scanning error rate {}",
        check_rules_pt1(&rules, &tickets)
    );

    let ir: Vec<Vec<Interval>> = compute_interval_range_pt2(&rules, tickets);

    println!("{:?}", ir[0]);
}

#[test]
fn interval_range_from_vec() {
    let r: Vec<_> = IntervalRange::from_vec(vec![1, 2, 3, 4, 6, 7]);
    assert_eq!(
        r,
        vec![Interval { from: 1, to: 4 }, Interval { from: 6, to: 7 }]
    );
}

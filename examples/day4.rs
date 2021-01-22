use itertools::Itertools;

struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        }
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }

    fn from_new_line(l: &'a str) -> Self {
        Passport::new().add_line(l)
    }

    fn add_line(mut self, l: &'a str) -> Self {
        l.split(' ').fold(self, |mut p , next| {
            // todo: fix unwrap
            let (f, value) = next.split(':').next_tuple().unwrap();

            match f {
                "byr" => p.byr = Some(value),
                "iyr" => p.iyr = Some(value),
                "eyr" => p.eyr = Some(value),
                "hgt" => p.hgt = Some(value),
                "hcl" => p.hcl = Some(value),
                "ecl" => p.ecl = Some(value),
                "pid" => p.pid = Some(value),
                "cid" => p.cid = Some(value),
                _ => (),
            };

            p
        })
    }

    fn from_lines(lines: impl Iterator<Item=&'a str>) -> Vec<Passport<'a>> {
        lines.into_iter().fold(
            (Vec::new(), None),
            |(mut v, mut current): (Vec<Passport>, Option<Passport>), next| {
                match current.take() {
                    Some(p) if next.is_empty() => {
                        if p.is_valid() {
                            v.push(p)
                        };
                        (v, None)
                    },
                    Some(mut p) => (v, Some(p.add_line(next))),
                    None => (v, Some(Passport::from_new_line(next))),
                }
        }).0
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input4").unwrap();

    let valid_passports = Passport::from_lines(inputs.split('\n'))
        .into_iter()
        .filter(|p| p.is_valid())
        .count();

    println!("Valid passports {}", valid_passports);
}

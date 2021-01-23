use itertools::Itertools;

enum Height {
    Cm(u16),
    Inches(u16),
}

impl Height {
    fn from_str(str: &str) -> Option<Self> {
        // todo: get rid of String
        let value = str.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse::<u16>().ok()?;
        match str.chars().skip_while(|c| c.is_ascii_digit()).collect::<String>().as_str() {
            "cm" if value >= 150 && value <= 193 => Some(Height::Cm(value)),
            "in" if value >= 59 && value <= 76 => Some(Height::Inches(value)),
            _ => None
        }
    }
}

struct BirthYear(u16);

// TODO: use generic to validate year
// fn validate_year<T>(str: &str, from: u16, to: u16) -> Option<T> {
//
// }

impl BirthYear {
    fn from_str(str: &str) -> Option<Self> {
        str.parse::<u16>()
            .ok()
            .filter(|&value| value >= 1920 && value <= 2002)
            .map(|value| BirthYear(value))
    }
}

struct IssueYear(u16);

impl IssueYear {
    fn from_str(str: &str) -> Option<Self> {
        str.parse::<u16>()
            .ok()
            .filter(|&value| value >= 2010 && value <= 2020)
            .map(|value| IssueYear(value))
    }
}


struct ExpireYear(u16);

impl ExpireYear {
    fn from_str(str: &str) -> Option<Self> {
        str.parse::<u16>()
            .ok()
            .filter(|&value| value >= 2020 && value <= 2030)
            .map(|value| ExpireYear(value))
    }
}

struct HairColor([char; 1]);

impl HairColor {
    fn from_str(str: &str) -> Option<Self> {
        // str.matches("[0-9a-f]{6}")
        Some(HairColor(['a']))
    }
}

enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl EyeColor {
    fn from_str(str: &str) -> Option<Self> {
        use EyeColor::*;
        match str {
            "amb" => Some(Amb),
            "blu" => Some(Blu),
            "brn" => Some(Brn),
            "gry" => Some(Gry),
            "grn" => Some(Grn),
            "hzl" => Some(Hzl),
            "oth" => Some(Oth),
            _ => None
        }
    }
}

struct PassportID(u32);

impl PassportID {
    fn from_str(str: &str) -> Option<Self> {
        if str.chars().take_while(|c| c.is_ascii_digit()).count() == 9 {
            Some(PassportID(str.parse::<u32>().ok()?))
        } else {
            None
        }
    }
}

struct Passport {
    byr: Option<BirthYear>,
    iyr: Option<IssueYear>,
    eyr: Option<ExpireYear>,
    hgt: Option<Height>,
    hcl: Option<HairColor>,
    ecl: Option<EyeColor>,
    pid: Option<PassportID>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
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

    fn from_new_line(l: &str) -> Self {
        l
            .split(&[' ', '\n'][..])
            .filter(|s| !s.is_empty())
            .fold(Passport::new(), |mut p, next| {
                // todo: fix unwrap
                let (f, value) = next.split(':').next_tuple().unwrap();

                match f {
                    "byr" => p.byr = BirthYear::from_str(value),
                    "iyr" => p.iyr = IssueYear::from_str(value),
                    "eyr" => p.eyr = ExpireYear::from_str(value),
                    "hgt" => p.hgt = Height::from_str(value),
                    "hcl" => p.hcl = HairColor::from_str(value),
                    "ecl" => p.ecl = EyeColor::from_str(value),
                    "pid" => p.pid = PassportID::from_str(value),
                    _ => (),
                };

                p
            })
    }

    fn from_lines<'a>(lines: impl Iterator<Item=&'a str>) -> Vec<Passport> {
        lines.into_iter().map(
            |next| Passport::from_new_line(next)
        ).collect()
    }
}

fn main() {
    let inputs = std::fs::read_to_string("inputs/input4").unwrap();

    let valid_passports = Passport::from_lines(inputs.split("\n\n"))
        .into_iter()
        .filter(|p| p.is_valid())
        .count();

    println!("Valid passports {}", valid_passports);
}

use crate::common::read_file_lines;

#[derive(Default, Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expriation_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

pub fn part1() {
    let passports = Passport::parse_from_file("input_4");
    println!("Valid passports: {}", passports.len());
}

impl Passport {
    pub fn parse_from_file(path: &str) -> Vec<Passport> {
        let lines = read_file_lines(path);
        let mut valid_passports = Vec::new();
        let mut work_passport = Self::default();
        for (line_num, line) in lines.iter().enumerate() {
            if line.len() == 0 {
                if work_passport.valid() {
                    // println!("Valid passport: {:?}\n", work_passport);
                    valid_passports.push(work_passport);
                } else {
                    println!("Invalid passport before line {}: {:?}\n", line_num, work_passport);
                }
                work_passport = Self::default();
                continue;
            }
            let fields : Vec<&str> = line.split(' ').collect();
            for field in fields {
                let name = &field[0..3];
                let value = &field[4..];
                // println!("{} | {}: {}", field, name, value);
                match name {
                    "byr" => work_passport.birth_year = Some(value.to_string()),
                    "iyr" => work_passport.issue_year = Some(value.to_string()),
                    "eyr" => work_passport.expriation_year = Some(value.to_string()),
                    "hgt" => work_passport.height = Some(value.to_string()),
                    "hcl" => work_passport.hair_color = Some(value.to_string()),
                    "ecl" => work_passport.eye_color = Some(value.to_string()),
                    "pid" => work_passport.passport_id = Some(value.to_string()),
                    "cid" => work_passport.country_id = Some(value.to_string()),
                    _ => panic!("Invalid field name"),
                }
            }
        }
        valid_passports
    }

    fn valid(&self) -> bool {
        if self.birth_year.is_none() {
            return false;
        }
        if self.issue_year.is_none() {
            return false;
        }
        if self.expriation_year.is_none() {
            return false;
        }
        if self.height.is_none() {
            return false;
        }
        if self.hair_color.is_none() {
            return false;
        }
        if self.eye_color.is_none() {
            return false;
        }
        if self.passport_id.is_none() {
            return false;
        }
        true
    }
}


use crate::common::read_file_lines;

#[derive(Default, Debug)]
struct Passport {
    birth_year:      Option<i32>,
    issue_year:      Option<i32>,
    expriation_year: Option<i32>,
    height:          Option<String>,
    hair_color:      Option<String>,
    eye_color:       Option<String>,
    passport_id:     Option<String>,
    country_id:      Option<String>,
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
        for  line in lines.iter() {
            if line.len() == 0 {
                if work_passport.valid() {
                    // println!("Valid passport: {:?}\n", work_passport);
                    valid_passports.push(work_passport);
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
                    "byr" => work_passport.birth_year = value.parse::<i32>().ok(),
                    "iyr" => work_passport.issue_year = value.parse::<i32>().ok(),
                    "eyr" => work_passport.expriation_year = value.parse::<i32>().ok(),
                    "hgt" => work_passport.height = Some(value.to_string()),
                    "hcl" => work_passport.hair_color = Some(value.to_string()),
                    "ecl" => work_passport.eye_color = Some(value.to_string()),
                    "pid" => work_passport.passport_id = Some(value.to_string()),
                    "cid" => work_passport.country_id = Some(value.to_string()),
                    _ => panic!("Invalid field name"),
                }
            }
        }
        if work_passport.valid() {
            // println!("Valid passport: {:?}\n", work_passport);
            valid_passports.push(work_passport);
        }
        valid_passports
    }

    fn valid(&self) -> bool {
        if let Some(year) = self.birth_year {
            if year < 1920 || year > 2002 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(year) = self.issue_year {
            if year < 2010 || year > 2020 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(year) = self.expriation_year {
            if year < 2020 || year > 2030 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(height) = &self.height {
            let len = height.len();
            let unit = &height[len-2..];
            let value = &height[..len-2];
            if unit != "in" && unit != "cm" {
                return false;
            }
            if let Ok(value) = value.parse::<i32>() {
                if unit == "in" {
                    if value < 59 || value > 76 {
                        println!("in out of bounds {}", value);
                        return false;
                    }
                } else if unit == "cm" {
                    if value < 150 || value > 193 {
                        println!("cm out of bounds {}", value);
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(color) = &self.hair_color {
            if color.chars().nth(0).unwrap() != '#' {
                return false;
            }
            for c in color[1..].bytes() {
                if (c < 48 && c > 57) || (c < 97 && c > 122) {
                    return false;
                }
            }
        } else {
            return false;
        }
        if let Some(color) = &self.eye_color {
            let mut valid = false;
            for c in vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] {
                if color == c {
                    valid = true;
                    break;
                }
            }
            if !valid {
                return false;
            }
        } else {
            return false;
        }
        if let Some(id) = &self.passport_id {
            if id.len() != 9 {
                return false;
            }
            if id.parse::<i32>().is_err() {
                return false;
            }
        } else {
            return false;
        }
        true
    }
}


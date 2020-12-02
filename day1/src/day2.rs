use crate::common::read_file_lines;

pub fn part1() {
    let lines = read_file_lines("input_2");
    let mut valid = 0;
    for line in lines {
        let mut parts = line.split_whitespace();
        let len = parts.next().unwrap();
        let mut len = len.split("-");
        let min_len = len.next().unwrap().parse::<i32>().unwrap();
        let max_len = len.next().unwrap().parse::<i32>().unwrap();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let pw = parts.next().unwrap();
        
        let mut letter_count = 0;
        for c in pw.chars() {
            if c == letter {
                letter_count += 1;
            }
        }
        if letter_count >= min_len && letter_count <= max_len {
            valid += 1;
        }
    }
    println!("Valid passwords: {}", valid);
}

pub fn part2() {
    let lines = read_file_lines("input_2");
    let mut valid = 0;
    for line in lines {
        let mut parts = line.split_whitespace();
        let index = parts.next().unwrap();
        let mut index = index.split("-");
        let first_index = index.next().unwrap().parse::<usize>().unwrap() - 1;
        let second_index = index.next().unwrap().parse::<usize>().unwrap() - 1;
        let letter = parts.next().unwrap().chars().next().unwrap();
        let pw = parts.next().unwrap();
        let mut chars = pw.chars();
        
        let mut correct_chars = 0;
        if let Some(c) = chars.nth(first_index) {
            if c == letter {
                correct_chars += 1;
            }
        }
        if let Some(c) = chars.nth(second_index - first_index - 1) {
            if c == letter {
                correct_chars += 1;
            }
        }
        if correct_chars == 1 {
            valid += 1;
            println!("Valid {},{},{}:{}",first_index, second_index, letter, pw);
        }
    }
    println!("Valid passwords: {}", valid);

}

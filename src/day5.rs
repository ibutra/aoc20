use crate::common::read_file_lines;

#[derive(Debug)]
struct BoardingPass {
    row: i32,
    column: i32,
}

impl BoardingPass {
    pub fn new(row: i32, column: i32) -> Self {
        Self {
            row: row,
            column: column,
        }
    }

    pub fn id(&self) -> i32 {
        self.row * 8 + self.column
    }
}

pub fn part1() {
    let lines = read_file_lines("input_5");
    let mut boarding_passes = Vec::new();
    let mut highest_id = 0;
    for line in lines {
        let row_str = &line[..7];
        let col_str = &line[7..];
        let col = calculate_col(col_str);
        let row = calculate_row(row_str);
        let boarding_pass = BoardingPass::new(row, col);
        println!("{:?}", boarding_pass);
        if boarding_pass.id() > highest_id {
            highest_id = boarding_pass.id();
        }
        boarding_passes.push(boarding_pass);
    }
    println!("Highest id: {}", highest_id);
}

pub fn part2() {
    let lines = read_file_lines("input_5");
    let mut boarding_passes = Vec::new();
    let mut highest_id = 0;
    for line in lines {
        let row_str = &line[..7];
        let col_str = &line[7..];
        let col = calculate_col(col_str);
        let row = calculate_row(row_str);
        let boarding_pass = BoardingPass::new(row, col);
        if boarding_pass.id() > highest_id {
            highest_id = boarding_pass.id();
        }
        boarding_passes.push(boarding_pass);
    }
    println!("Missing id: {}", find_missing_id(&boarding_passes));
}

fn find_missing_id(passes: &Vec<BoardingPass>) -> i32 {
    let ids : Vec<i32> = passes.iter().map(|x| x.id()).collect();
    for r in 0..=127 {
        for c in 0..=7 {
            let id = r * 8 + c;
            if ids.contains(&id) {
                println!("found id: {}", id);
                continue;
            }
            if ids.contains(&(id - 1)) && ids.contains(&(id + 1)) {
                return id;
            }
            println!("Surrounding ids found: {}", id);
        }
    }
    0
}


fn calculate_row(code: &str) -> i32 {
    let mut high = 127;
    let mut low = 0;
    for c in code.chars() {
        // println!("high: {}, low: {} -> {} ->", high, low, c);
        let diff = (high - low) / 2 + 1;
        if c == 'F' {
            high -= diff;
        } else {
            low += diff;
        }
    }
    high
}

fn calculate_col(code: &str) -> i32 {
    let mut high = 7;
    let mut low = 0;
    for c in code.chars() {
        let diff = (high - low) / 2 + 1;
        if c == 'L' {
            high -= diff;
        } else {
            low += diff;
        }
    }
    high
}

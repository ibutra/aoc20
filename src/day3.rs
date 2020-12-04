use crate::common::read_file_lines;

pub fn part1() {
    let count = calculate_trees_on_slope(3, 1);
    println!("Trees on route: {}", count);
}

pub fn part2() {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product : i64 = 1;
    for (x, y) in slopes {
        let count : i64 = calculate_trees_on_slope(x, y).into();
        println!("Count: {}", count);
        product = product * count;
    }
    println!("Product: {}", product);
}

pub fn calculate_trees_on_slope(x_slope: usize, y_slope: usize) -> i32 {
    let lines = read_file_lines("input_3");
    let mut trees: Vec<Vec<bool>> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        trees.push(Vec::new());
        for c in line.chars() {
            if c == '#' {
                trees[y].push(true);
            } else {
                trees[y].push(false);
            }
        }
    }

    let mut x = 0;
    let mut y = 0;
    let height = trees.len();
    let width = trees[0].len();
    let mut count = 0;
    loop {
        if trees[y][x] {
            count += 1;
        }
        x = (x + x_slope) % width;
        y += y_slope;
        if y >= height {
            break;
        }
    }
    count
}

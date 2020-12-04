use crate::common::read_file_lines;

pub fn part1() {
    let lines = read_file_lines("input_1");
    let numbers : Vec<i32> = lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let len = numbers.len();
    for a in 0..len {
        for b in a+1..len {
            let num_a = numbers[a];
            let num_b = numbers[b];
            if num_a + num_b == 2020 {
                println!("Day1 Part1 Solution: {}", num_a * num_b);
            }
        }
    }
}

pub fn part2() {
    let lines = read_file_lines("input_1");
    let numbers : Vec<i32> = lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let len = numbers.len();
    for a in 0..len-2 {
        for b in a+1..len-1 {
            for c in b+1..len {
                let num_a = numbers[a];
                let num_b = numbers[b];
                let num_c = numbers[c];
                if num_a + num_b + num_c == 2020 {
                    println!("Day1 Part2 Solution: {}", num_a * num_b * num_c);
                }
            }
        }
    }

}

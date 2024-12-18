use std::fs::read_to_string;

fn is_list_safe(list: &Vec<i32>, is_increment: bool) -> bool {
    for (i, num) in list.iter().take(list.len() - 1).enumerate() {
        let diff = list[i + 1] - num;
        if is_increment && (diff <= 0 || diff > 3) || !is_increment && (diff >= 0 || diff < -3) {
            return false;
        }
    }
    return true;
}
fn is_safe(list: &Vec<i32>, has_dampener: bool) -> bool {
    let is_increment = list[1] - list[0] > 0;
    if !has_dampener {
        return is_list_safe(list, is_increment);
    } else {
        return true;
    }
}

pub fn solution() {
    let mut count = 0;
    for line in read_to_string("/home/baobao/dev/projects/aoc2024/src/two_input.txt")
        .unwrap()
        .lines()
    {
        let line_str = line.to_string();
        let list: Vec<i32> = line_str
            .split_whitespace()
            .map(|i| i.parse::<i32>())
            .map(Result::unwrap)
            .collect();
        if is_safe(&list, false) {
            count += 1;
        }
    }
    println!("Safe reports: {}", count);
}

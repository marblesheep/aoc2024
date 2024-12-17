use std::collections::HashMap;
use std::fs::read_to_string;

pub fn solution() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in read_to_string("/home/baobao/dev/projects/aoc2024/src/one_input.txt")
        .unwrap()
        .lines()
    {
        let line_str = line.to_string();
        let mut split = line_str.split_whitespace();
        list1.push(split.next().expect("success").to_string().parse().unwrap());
        list2.push(split.next().expect("success").to_string().parse().unwrap());
    }

    list1.sort();
    list2.sort();
    let mut sum = 0;
    for (pos, e) in list1.iter().enumerate() {
        sum += (e - list2[pos]).abs();
    }

    println!("total distance: {}", sum);
}

pub fn solution2() {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for line in read_to_string("/home/baobao/dev/projects/aoc2024/src/one_input.txt")
        .unwrap()
        .lines()
    {
        let line_str = line.to_string();
        let mut split = line_str.split_whitespace();
        let elem1 = split.next().expect("ok").to_string();
        let elem2 = split.next().expect("ok").to_string();

        match map1.get(&elem1) {
            Some(count) => {
                map1.insert(elem1, count + 1);
            }
            None => {
                map1.insert(elem1, 1);
            }
        }
        match map2.get(&elem2) {
            Some(count) => {
                map2.insert(elem2, count + 1);
            }
            None => {
                map2.insert(elem2, 1);
            }
        }
    }

    let mut similarity_score = 0;
    for (key, value) in map1.into_iter() {
        let count = match map2.get(&key) {
            Some(val) => val,
            None => &0,
        };
        if *count > 0 {
            let key_val: i32 = key.parse().unwrap();
            similarity_score += key_val * value * count;
        }
    }
    println!("total similarity: {}", similarity_score);
}

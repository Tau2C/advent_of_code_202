use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("day01/input.txt").unwrap();

    let (distance, lists) = puzzle1(&input);
    let score = puzzle2(&input, lists);
    println!("Distance: {}\nScore: {}", distance, score);
}

fn puzzle1(input: &str) -> (u32, (Vec<u32>, Vec<u32>)) {
    let numbers: Vec<u32> = input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    let mut list_l = Vec::new();
    let mut list_r = Vec::new();

    for (index, value) in numbers.into_iter().enumerate() {
        if index % 2 == 0 {
            list_l.push(value);
        } else {
            list_r.push(value);
        }
    }
    list_l.sort();
    list_r.sort();

    let mut distance = 0;

    for i in 0..list_l.len() {
        distance += list_r[i].abs_diff(list_l[i]);
    }
    (distance, (list_l, list_r))
}

fn puzzle2(_input: &str, lists: (Vec<u32>, Vec<u32>)) -> u32 {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for i in 0..lists.1.len() {
        let item = lists.1[i];
        let count = match counts.get(&item) {
            Some(val) => val.clone(),
            _ => 0,
        };
        counts.insert(item, count+1);
    }

    let mut score: u32 = 0;

    for i in 0..lists.0.len() {
        let multiplier = match counts.get(&lists.0[i]) {
            Some(val) => val.clone(),
            _ => 0,
        };
        score += lists.0[i] * multiplier;
    }
    score
}

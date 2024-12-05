use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = std::fs::read_to_string("day05/input.txt").unwrap();

    let (rules, updates) = parse_input(&input);

    let p1 = puzzle1(&rules, &updates);
    println!("P1: {p1}");
    let p2 = puzzle2(&rules, &updates);
    println!("P2: {p2}");
}

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for l in input.split("\n\n").nth(0).unwrap().split('\n') {
        let mut a = l.trim().split('|');
        let p1 = a.next().unwrap().parse::<u32>().unwrap();
        let p2 = a.next().unwrap().parse::<u32>().unwrap();

        if let Some(list) = rules.get_mut(&p1) {
            list.push(p2);
        } else {
            let mut v = Vec::new();
            v.push(p2);
            rules.insert(p1, v);
        }
    }

    let updates: Vec<Vec<u32>> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .split('\n')
        .map(|l| {
            l.trim()
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    (rules, updates)
}

fn puzzle1(rules: &HashMap<u32, Vec<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    let mut correct_ordered_updates = Vec::new();
    for update in updates {
        let mut pages_updated = Vec::new();
        for page in update {
            if let Some(list) = rules.get(page) {
                let mut was_printed = false;
                for page in list {
                    if pages_updated.contains(page) {
                        was_printed = true;
                    }
                }

                if was_printed {
                    break;
                }
            }
            pages_updated.push(*page);
        }
        if update.len() == pages_updated.len() {
            correct_ordered_updates.push(update);
        }
    }

    correct_ordered_updates
        .iter()
        .map(|update| *update.get(update.len() / 2).unwrap())
        .sum::<u32>()
}

fn puzzle2(rules: &HashMap<u32, Vec<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    let mut incorrect_ordered_updates = Vec::new();
    for update in updates {
        let mut pages_updated = Vec::new();
        for page in update {
            if let Some(list) = rules.get(page) {
                let mut was_printed = false;
                for page in list {
                    if pages_updated.contains(page) {
                        was_printed = true;
                    }
                }

                if was_printed {
                    break;
                }
            }
            pages_updated.push(*page);
        }
        if update.len() != pages_updated.len() {
            incorrect_ordered_updates.push(update);
        }
    }

    incorrect_ordered_updates
        .iter()
        .map(|update| {
            let mut ordered_update = update.to_vec();
            ordered_update.sort_by(|a, b| {
                for pair in rules.iter().flat_map(|f| {
                    let mut pairs = Vec::new();
                    for y in f.1 {
                        pairs.push((f.0, y));
                    }
                    pairs
                }) {
                    if a == pair.0 && b == pair.1 {
                        return Ordering::Less;
                    }
                    if a == pair.1 && b == pair.0 {
                        return Ordering::Greater;
                    }
                }
                return Ordering::Equal;
            });
            ordered_update
        })
        .map(|update| *update.get(update.len() / 2).unwrap())
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = std::fs::read_to_string("example.txt").unwrap();

        let (rules, updates) = parse_input(&input);

        assert_eq!(puzzle1(&rules, &updates), 143)
    }

    #[test]
    fn example2() {
        let input = std::fs::read_to_string("example.txt").unwrap();

        let (rules, updates) = parse_input(&input);

        assert_eq!(puzzle2(&rules, &updates), 123)
    }
}

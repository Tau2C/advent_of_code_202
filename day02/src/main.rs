fn main() {
    let input = std::fs::read_to_string("day02/input.txt").unwrap();

    let p1 = puzzle1(&input);
    println!("P1: {p1}");
    let p2 = puzzle2(&input);
    println!("P2: {p2}");
}

enum Monotonicity {
    Decreasing,
    Increasing,
}

fn check_if_safe(raport: &Vec<u32>) -> bool {
    if raport[0] == raport[1] {
        return false;
    }
    let monotonicity = if raport[0] < raport[1] {
        Monotonicity::Increasing
    } else {
        Monotonicity::Decreasing
    };

    let pairs = raport.windows(2).collect::<Vec<&[u32]>>().len();
    let filtered_pairs = raport
        .windows(2)
        .filter(|w| match monotonicity {
            Monotonicity::Decreasing => {
                if w[0] > w[1] {
                    true
                } else {
                    false
                }
            }
            Monotonicity::Increasing => {
                if w[0] < w[1] {
                    true
                } else {
                    false
                }
            }
        })
        .filter(|w| w[0].abs_diff(w[1]) <= 3)
        .collect::<Vec<&[u32]>>()
        .len();
    pairs == filtered_pairs
}

fn puzzle1(input: &str) -> u32 {
    let raports = input
        .split_terminator('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut safe_raports = 0;

    for raport in &raports {
        if check_if_safe(raport) {
            safe_raports += 1;
        }
    }
    return safe_raports;
}

fn puzzle2(input: &str) -> i32 {
    let raports = input
        .split_terminator('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut safe_raports = 0;

    for raport in &raports {
        if check_if_safe(raport) {
            safe_raports += 1;
            continue;
        }
        for i in 0..raport.len() {
            let mut a = raport.clone();
            _ = a.remove(i);
            if check_if_safe(&a) {
                safe_raports += 1;
                break;
            }
        }
    }
    return safe_raports;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("/Data/Projects/advent_of_code_2024/day02/example.txt")
            .unwrap();
        assert_eq!(puzzle2(&input), 4);
    }
}

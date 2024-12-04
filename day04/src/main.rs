fn main() {
    let input = std::fs::read_to_string("day04/input.txt").unwrap();

    let p1 = puzzle1(&input);
    println!("P1: {p1}");
    let p2 = puzzle2(&input);
    println!("P2: {p2}");
}

fn puzzle1(input: &str) -> u32 {
    let mut xmas_num = 0;

    let input = input.split('\n').collect::<Vec<&str>>();

    let input_y = input.len();
    let input_x = input[0].len();

    for y in 0..input_y {
        for x in 0..input_x {
            let top = y >= 3;
            let bottom = input_y - y > 3;
            let left = x >= 3;
            let right = input_x - x > 3;

            if left {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y].chars().nth(x - 1).unwrap() == 'M'
                    && input[y].chars().nth(x - 2).unwrap() == 'A'
                    && input[y].chars().nth(x - 3).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
            if right {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y].chars().nth(x + 1).unwrap() == 'M'
                    && input[y].chars().nth(x + 2).unwrap() == 'A'
                    && input[y].chars().nth(x + 3).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
            if top {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y - 1].chars().nth(x).unwrap() == 'M'
                    && input[y - 2].chars().nth(x).unwrap() == 'A'
                    && input[y - 3].chars().nth(x).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
            if bottom {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y + 1].chars().nth(x).unwrap() == 'M'
                    && input[y + 2].chars().nth(x).unwrap() == 'A'
                    && input[y + 3].chars().nth(x).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
            if left && top {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y - 1].chars().nth(x - 1).unwrap() == 'M'
                    && input[y - 2].chars().nth(x - 2).unwrap() == 'A'
                    && input[y - 3].chars().nth(x - 3).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
            if right && top {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y - 1].chars().nth(x + 1).unwrap() == 'M'
                    && input[y - 2].chars().nth(x + 2).unwrap() == 'A'
                    && input[y - 3].chars().nth(x + 3).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
            if left && bottom {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y + 1].chars().nth(x - 1).unwrap() == 'M'
                    && input[y + 2].chars().nth(x - 2).unwrap() == 'A'
                    && input[y + 3].chars().nth(x - 3).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
            if right && bottom {
                if input[y].chars().nth(x).unwrap() == 'X'
                    && input[y + 1].chars().nth(x + 1).unwrap() == 'M'
                    && input[y + 2].chars().nth(x + 2).unwrap() == 'A'
                    && input[y + 3].chars().nth(x + 3).unwrap() == 'S'
                {
                    xmas_num += 1;
                }
            }
        }
    }
    xmas_num
}

fn puzzle2(input: &str) -> i32 {
    let mut xmas_num = 0;

    let input = input.split('\n').collect::<Vec<&str>>();

    let input_y = input.len();
    let input_x = input[0].len();

    for y in 1..(input_y - 1) {
        for x in 1..(input_x - 1) {
            if input[y].chars().nth(x).unwrap() != 'A' {
                continue;
            }
            if (input[y - 1].chars().nth(x - 1).unwrap() == 'M'
                && input[y + 1].chars().nth(x + 1).unwrap() == 'S')
                || (input[y - 1].chars().nth(x - 1).unwrap() == 'S'
                    && input[y + 1].chars().nth(x + 1).unwrap() == 'M')
            {
                if (input[y + 1].chars().nth(x - 1).unwrap() == 'M'
                    && input[y - 1].chars().nth(x + 1).unwrap() == 'S')
                    || (input[y + 1].chars().nth(x - 1).unwrap() == 'S'
                        && input[y - 1].chars().nth(x + 1).unwrap() == 'M')
                {
                    xmas_num += 1;
                }
            }
        }
    }
    xmas_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_puzzle1() {
        let example = std::fs::read_to_string("example.txt").unwrap();
        assert_eq!(puzzle1(&example), 18)
    }

    #[test]
    fn example_puzzle2() {
        let example = std::fs::read_to_string("example.txt").unwrap();
        assert_eq!(puzzle2(&example), 9)
    }
}

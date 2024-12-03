use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("day03/input.txt").unwrap();

    let p1 = puzzle1(&input);
    println!("P1: {p1}");
    let p2 = puzzle2(&input);
    println!("P2: {p2}");
}

fn puzzle1(input: &str) -> u64 {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    for (_, [num1, num2]) in mul_regex.captures_iter(input).map(|c| c.extract()) {
        result += num1.parse::<u64>().unwrap() * num2.parse::<u64>().unwrap();
    }
    result
}

fn puzzle2(input: &str) -> u64 {
    let dont_regex = Regex::new(r"(don't\(\))").unwrap();
    let do_regex = Regex::new(r"(do\(\))").unwrap();
    let mul_regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut result = 0;

    let mut do_ranges: Vec<usize> = Vec::new();
    let mut dont_ranges: Vec<usize> = Vec::new();

    for a in dont_regex.captures_iter(input).map(|c| c.get(0)) {
        if a.is_none() {
            continue;
        }
        dont_ranges.push(a.unwrap().start());
    }

    for a in do_regex.captures_iter(input).map(|c| c.get(0)) {
        if a.is_none() {
            continue;
        }
        do_ranges.push(a.unwrap().start());
    }

    let do_range = do_ranges;
    let dont_range = dont_ranges;
    let mut ranges: Vec<(bool, usize, usize)> = Vec::new();

    ranges.push((true, 0, input.len()));

    let mut do_index = 0;
    let mut dont_index = 0;

    while do_index < do_range.len() || dont_index < dont_range.len() {
        let new = if do_index < do_range.len() && dont_index < dont_range.len() {
            if do_range[do_index] < dont_range[dont_index] {
                do_index += 1;
                (true, do_range[do_index - 1])
            } else {
                dont_index += 1;
                (false, dont_range[dont_index - 1])
            }
        } else if do_index < do_range.len() {
            do_index += 1;
            (true, do_range[do_index - 1])
        } else {
            dont_index += 1;
            (false, dont_range[dont_index - 1])
        };

        let old = ranges.pop().unwrap();

        if old.0 != new.0 {
            ranges.push((old.0, old.1, new.1));
            ranges.push((new.0, new.1, old.2));
        } else {
            ranges.push(old);
        }
    }

    let ranges = ranges;
    let mut muls = Vec::new();

    for (loc, (_, [_, num1, num2])) in mul_regex
        .captures_iter(input)
        .map(|c| (c.get(0).unwrap(), c.extract()))
    {
        muls.push((
            loc.start(),
            num1.parse::<u64>().unwrap(),
            num2.parse::<u64>().unwrap(),
        ));
    }

    for mul in muls {
        for range in &ranges {
            if range.1 < mul.0 && mul.0 < range.2 {
                if range.0 {
                    result += mul.1 * mul.2;
                }
                break;
            }
        }
    }
    result
}

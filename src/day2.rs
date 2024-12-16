#![allow(dead_code, unused_variables, unused_mut)]

type Input = Vec<Vec<u32>>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    let mut result = Vec::new();
    input
        .lines()
        .for_each(|l| {
            let mut entry = Vec::new();
            for elem in l.trim().split_whitespace() {
                if let Ok(value) = elem.parse() {
                    entry.push(value);
                }
            }
            result.push(entry);
        });
    
    result
}

#[derive(Debug, PartialEq)]
enum Check {
    Safe,
    Unsafe
}

fn check_safety(entry: &Vec<u32>) -> Check {
    let mut entries = entry.iter();
    let mut first = entries.next().unwrap();
    let mut last = entries.next().unwrap();

    if first == last || first.abs_diff(*last) > 3 {
        return Check::Unsafe;
    }

    let mut increasing = first < last;
    
    while let Some(cur) = entries.next() {
        if increasing && ((cur <= last) || (cur - last) > 3) {
            return Check::Unsafe;
        } else if !increasing && ((cur >= last) || (last - cur) > 3) {
            return Check::Unsafe;
        }
        last = cur;
    }

    return Check::Safe;
}

fn check_safety_part2(entry: &Vec<u32>) -> Check {
    for (idx, _) in entry.iter().enumerate() {
        let mut entry_2 = entry.clone();
        entry_2.remove(idx);

        if let Check::Safe = check_safety(&entry_2) {
            return Check::Safe;
        }
    }
    return Check::Unsafe;
}
/*
fn check_safety_part2(entry: &Vec<u32>) -> Check {
    let mut entry2 = entry.clone();
    entry2.remove(0);
    if let Check::Safe = check_safety(&entry2) {
        return Check::Safe;
    }
    
    let mut entries = entry.iter();
    let mut first = entries.next().unwrap();
    let mut prev = entries.next().unwrap();
    let mut fails = 0;

    if first == prev || first.abs_diff(*prev) > 3 {
        fails +=1;
        match entries.next() {
            None => { return Check::Safe },
            Some(val) => { prev = val },
        }

        if first == prev || first.abs_diff(*prev) > 3 {
            return Check::Unsafe;
        }
    }

    let mut increasing = first < prev;
    
    while let Some(cur) = entries.next() {
        if increasing && ((cur <= prev) || (cur - prev) > 3) {
            fails += 1;
        } else if !increasing && ((cur >= prev) || (prev - cur) > 3) {
            fails += 1;
        } else {
            prev = cur;
        }

        if fails > 1 {
            return Check::Unsafe;
        }
    }

    return Check::Safe;
}
*/


#[aoc(day2, part1)]
fn part1(input: &Input) -> u32 {
    let mut result = 0;

    input.iter().for_each(|entry| {
        match check_safety(entry) {
            Check::Safe => result += 1,
            Check::Unsafe => (),
        }
    });
    
    return result;
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> u32 {
    let mut result = 0;

    input.iter().for_each(|entry| {
        match check_safety_part2(entry) {
            Check::Safe => result += 1,
            Check::Unsafe => (),
        }
    });
    
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let input = parse("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9");
        assert_eq!(input,
                   vec![vec![7, 6, 4, 2, 1],
                        vec![1, 2, 7, 8, 9],
                        vec![9, 7, 6, 2, 1],
                        vec![1, 3, 2, 4, 5],
                        vec![8, 6, 4, 4, 1],
                        vec![1, 3, 6, 7, 9]]);
    }

    #[test]
    fn test_check_safety() {
        assert_eq!(check_safety(&vec![7, 6, 4, 2, 1]), Check::Safe);
        assert_eq!(check_safety(&vec![1, 2, 7, 8, 9]), Check::Unsafe);
        assert_eq!(check_safety(&vec![9, 7, 6, 2, 1]), Check::Unsafe);
        assert_eq!(check_safety(&vec![1, 3, 2, 4, 5]), Check::Unsafe);
        assert_eq!(check_safety(&vec![8, 6, 4, 4, 1]), Check::Unsafe);
        assert_eq!(check_safety(&vec![1, 3, 6, 7, 9]), Check::Safe);

        assert_eq!(check_safety(&vec![1, 2, 3, 4, 5]), Check::Safe);
        assert_eq!(check_safety(&vec![5, 4, 3, 2, 1]), Check::Safe);
        assert_eq!(check_safety(&vec![1, 2, 3, 4, 5, 5]), Check::Unsafe);
        assert_eq!(check_safety(&vec![5, 4, 3, 2, 1, 1]), Check::Unsafe);

        assert_eq!(check_safety(&vec![1, 2, 3, 4, 5, 9]), Check::Unsafe);
        assert_eq!(check_safety(&vec![5, 1]), Check::Unsafe);
        assert_eq!(check_safety(&vec![1, 9]), Check::Unsafe);
    }
    
    #[test]
    fn part1_example() {
        let input =
            vec![
                vec![7, 6, 4, 2, 1], // Safe
                vec![1, 2, 7, 8, 9], // Unsafe
                vec![9, 7, 6, 2, 1], // Unsafe
                vec![1, 3, 2, 4, 5], // Unsafe 
                vec![8, 6, 4, 4, 1], // Unsafe
                vec![1, 3, 6, 7, 9], // Safe
            ];

        assert_eq!(2, part1(&input));
    }

    #[test]
    fn test_check_safety_part2() {
        assert_eq!(check_safety_part2(&vec![7, 6, 4, 2, 1]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![1, 2, 7, 8, 9]), Check::Unsafe);
        assert_eq!(check_safety_part2(&vec![9, 7, 6, 2, 1]), Check::Unsafe);
        assert_eq!(check_safety_part2(&vec![1, 3, 2, 4, 5]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![8, 6, 4, 4, 1]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![1, 3, 6, 7, 9]), Check::Safe);

        assert_eq!(check_safety_part2(&vec![1, 2, 3, 4, 5]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![5, 4, 3, 2, 1]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![1, 2, 3, 4, 5, 5]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![5, 4, 3, 2, 1, 1]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![1, 2, 3, 4, 5, 5, 5]), Check::Unsafe);
        assert_eq!(check_safety_part2(&vec![5, 4, 3, 2, 1, 1, 1]), Check::Unsafe);

        assert_eq!(check_safety_part2(&vec![5, 10, 4, 3, 2, 1]), Check::Safe);

        assert_eq!(check_safety_part2(&vec![1, 2, 3, 4, 5, 9]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![1, 1, 5, 6, 7, 8]), Check::Unsafe);

        assert_eq!(check_safety_part2(&vec![10, 5, 6, 7, 8, 9]), Check::Safe);

        assert_eq!(check_safety_part2(&vec![75, 77, 72, 70, 69,]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![28, 28, 27, 26, 23,]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![20, 16, 14, 12, 10, 8, 7, 6]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![74, 70, 71, 70, 68, 65]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![44, 37, 34, 31, 30,]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![52, 47, 49, 46, 43, 41, 40]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![59, 56, 53, 50, 47, 47, 44, 41]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![70, 71, 73, 74, 75, 78, 79, 83]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![32, 35, 33, 34, 35, 38]), Check::Safe);
        assert_eq!(check_safety_part2(&vec![51, 50, 47, 45, 42, 41, 34]), Check::Safe);
    }
    
    #[test]
    fn part2_example() {
        let input =
            vec![
                vec![7, 6, 4, 2, 1], // Safe
                vec![1, 2, 7, 8, 9], // Unsafe
                vec![9, 7, 6, 2, 1], // Unsafe
                vec![1, 3, 2, 4, 5], // Safe 
                vec![8, 6, 4, 4, 1], // Safe
                vec![1, 3, 6, 7, 9], // Safe
            ];

        assert_eq!(4, part2(&input));
    }
}

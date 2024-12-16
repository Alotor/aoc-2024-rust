#![allow(unused_variables, unused_mut, dead_code)]

type Input = Vec<(i64, Vec<i64>)>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let ss: Vec<&str> = l.trim().split(": ").collect();
            let target = ss[0].parse().unwrap();
            let ns: Vec<i64> = ss[1].split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
            (target, ns)
        })
        .collect()
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}

fn solver(target: &i64, nums: &Vec<i64>) -> bool {

    fn solve_idx(target: &i64, nums: &Vec<i64>, acc: i64, idx: usize) -> bool {
        if acc > *target {
            // The accumulated is greater than the target we don't need to
            // investigate further
            return false
        }
        
        if let Some(val) = nums.get(idx) {
            solve_idx(target, nums, acc + val, idx + 1)
                || solve_idx(target, nums, acc * val, idx + 1)
        } else {
            // end we check if the value is the correct one
            acc == *target
        }
    }

    solve_idx(target, nums, 0, 0)
}

fn solver_part2(target: &i64, nums: &Vec<i64>) -> bool {
    fn solve_idx(target: &i64, nums: &Vec<i64>, acc: i64, idx: usize) -> bool {
        if acc > *target {
            // The accumulated is greater than the target we don't need to
            // investigate further
            return false
        }
        
        if let Some(val) = nums.get(idx) {
            solve_idx(target, nums, acc + val, idx + 1)
                || solve_idx(target, nums, acc * val, idx + 1)
                || solve_idx(target, nums, concat(acc, *val), idx + 1)
        } else {
            // end we check if the value is the correct one
            acc == *target
        }
    }

    solve_idx(target, nums, 0, 0)
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> i64 {
    input.iter().map(|(target, nums)| {
        if solver(&target, &nums) {
            *target
        } else {
            0
        }
    }).sum()
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> i64 {
    input.iter().map(|(target, nums)| {
        if solver_part2(&target, &nums) {
            *target
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = parse("190: 10 19
                     3267: 81 40 27
                     83: 17 5
                     156: 15 6
                     7290: 6 8 6 15
                     161011: 16 10 13
                     192: 17 8 14
                     21037: 9 7 18 13
                     292: 11 6 16 20");

        assert_eq!(input[0].0, 190);
        assert_eq!(input[0].1, vec![10, 19]);

        assert_eq!(input[8].0, 292);
        assert_eq!(input[8].1, vec![11, 6, 16, 20]);
    }

    #[test]
    fn test_solver() {
        assert!(solver(&190, &vec![10, 19]));
        assert!(solver(&3267, &vec![81, 40, 27]));
        assert!(solver(&292, &vec![11, 6, 16, 20]));

        assert!(!solver(&156, &vec![15, 6]));
        assert!(!solver(&7290, &vec![6, 8, 6, 15]));
        assert!(!solver(&192, &vec![17, 8, 14]));
    }

    #[test]
    fn test_part1() {
        let input = parse("190: 10 19
                     3267: 81 40 27
                     83: 17 5
                     156: 15 6
                     7290: 6 8 6 15
                     161011: 16 10 13
                     192: 17 8 14
                     21037: 9 7 18 13
                     292: 11 6 16 20");

        assert_eq!(part1(&input), 3749);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(15, 6), 156);
    }

    #[test]
    fn test_solver_part2() {
        assert!(solver_part2(&190, &vec![10, 19]));
        assert!(solver_part2(&3267, &vec![81, 40, 27]));
        assert!(solver_part2(&292, &vec![11, 6, 16, 20]));

        assert!(solver_part2(&156, &vec![15, 6]));
        assert!(solver_part2(&7290, &vec![6, 8, 6, 15]));
        assert!(solver_part2(&192, &vec![17, 8, 14]));
    }

    #[test]
    fn test_part2() {
        let input = parse("190: 10 19
                     3267: 81 40 27
                     83: 17 5
                     156: 15 6
                     7290: 6 8 6 15
                     161011: 16 10 13
                     192: 17 8 14
                     21037: 9 7 18 13
                     292: 11 6 16 20");

        assert_eq!(part2(&input), 11387);
    }
}

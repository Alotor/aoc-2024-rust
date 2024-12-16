#![allow(unused_variables, unused_mut, dead_code, unused_comparisons)]

use memoize::memoize;

type Input = Vec<u64>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Input {
    input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn even_digits(elem: u64) -> bool {
    (elem.ilog10() + 1) % 2 == 0
}

fn split_digits(elem: u64) -> (u64, u64) {
    let ndig = elem.ilog10() + 1;

    let last  = elem % 10u64.pow(ndig / 2);
    let first = elem / 10u64.pow(ndig / 2);

    (first, last)
}

#[memoize]
fn loop_stone(value: u64, loops: u32) -> u64 {
    if loops == 0 {
        1
    } else if value == 0 {
        loop_stone(1, loops - 1)
    } else if even_digits(value) {
        let (left, right) = split_digits(value);
        loop_stone(left, loops - 1) + loop_stone(right, loops - 1)
    } else {
        loop_stone(value * 2024, loops - 1)
    }
}

// First solution. Too slow for part 2
//
// fn iterate_stones(input: &Input) -> Input {
//     let mut result: Input = Vec::with_capacity(input.len() * 2);
// 
//     for elem in input {
//         if *elem == 0 {
//             result.push(1);
//         } else if even_digits(*elem) {
//             let (left, right) = split_digits(*elem);
//             result.push(left);
//             result.push(right);
//         } else {
//             result.push(*elem * 2024);
//         }
//     }
//     
//     return result
// }

#[aoc(day11, part1)]
fn part1(input: &Input) -> u64 {
    let mut stones = input.clone();

    input
        .iter()
        .map(|v| loop_stone(*v, 25))
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &Input) -> u64 {
    let mut stones = input.clone();

    input
        .iter()
        .map(|v| loop_stone(*v, 75))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = parse("0 1 10 99 999");
        assert_eq!(input, vec![0, 1, 10, 99, 999]);
    }

    // #[test]
    // fn test_iter_stones_1() {
    //     assert_eq!(iterate_stones(&vec![125, 17]), vec![253000, 1, 7]);
    //     assert_eq!(iterate_stones(&vec![253000, 1, 7]), vec![253, 0, 2024, 14168]);
    //     assert_eq!(iterate_stones(&vec![253, 0, 2024, 14168]), vec![512072, 1, 20, 24, 28676032]);
    //     assert_eq!(iterate_stones(&vec![512072, 1, 20, 24, 28676032]), vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
    //     assert_eq!(iterate_stones(
    //         &vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
    //         vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]);
    //     assert_eq!(iterate_stones(
    //         &vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]),
    //         vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]);
    // }

    #[test]
    fn test_even_digits() {
        assert!(even_digits(10));
        assert!(!even_digits(101));
        assert!(even_digits(1010));
        assert!(!even_digits(10101));
    }

    #[test]
    fn test_split_digits() {
        assert_eq!(split_digits(123456), (123, 456));
        assert_eq!(split_digits(240004), (240, 4));
        assert_eq!(split_digits(17), (1, 7));
    }

    #[test]
    fn test_part1() {
        let input = parse("125 17");
        assert_eq!(part1(&input), 55312);
    }
}

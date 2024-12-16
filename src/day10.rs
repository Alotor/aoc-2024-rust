#![allow(unused_variables, unused_mut, dead_code, unused_comparisons)]

use std::collections::HashSet;
use crate::utils::get_m;

type Input = Vec<Vec<u32>>;

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(99))
                .collect()
        })
        .collect()
}

fn next_cells(input: &Input, i: usize, j: usize) -> Vec<(usize, usize)> {
    let target_val = input[i][j] + 1;
    let i = i as i32;
    let j = j as i32;

    vec![(i - 1, j),
         (i,     j + 1),
         (i + 1, j),
         (i,     j - 1)]
        .iter()
        .map(|(i, j)| {
            let v: u32 = *get_m(input, *i, *j, &99);
            (i, j, v)
        })
        .filter(|(_, _, v)| *v == target_val)
        .map(|(i, j, _)| (*i as usize, *j as usize))
        .collect()
}

fn score_trailhead(input: &Input, i: usize, j: usize) -> usize {
    fn score_trailhead_rec(input: &Input, i: usize, j: usize) -> Vec<(usize, usize)> {
        if input[i][j] == 9 {
            vec![(i, j)]
        } else {
            let next_cells: Vec<(usize, usize)> = next_cells(input, i, j);
            next_cells.iter().flat_map(|(i, j)| {
                score_trailhead_rec(input, *i, *j)
            }).collect()
        }
    }

    let targets = score_trailhead_rec(input, i, j);
    let positions = HashSet::<(usize, usize)>::from_iter(targets);
    positions.len()
}

fn rate_trailhead(input: &Input, i: usize, j: usize) -> usize {
    if input[i][j] == 9 {
        1
    } else {
        let next_cells: Vec<(usize, usize)> = next_cells(input, i, j);
        next_cells.iter().map(|(i, j)| {
            rate_trailhead(input, *i, *j)
        }).sum()
    }
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> i32 {
    let mut acc = 0;
    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            if input[i][j] == 0 {
                acc += score_trailhead(input, i, j) as i32
            }
        }
    }
    acc
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> i32 {
    let mut acc = 0;
    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            if input[i][j] == 0 {
                acc += rate_trailhead(input, i, j) as i32
            }
        }
    }
    acc
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "8901
                     7.12
                     8743
                     9654";
        let result = parse(input);
        assert_eq!(result,
                   vec![
                       vec![8, 9, 0, 1],
                       vec![7, 99, 1, 2],
                       vec![8, 7, 4, 3],
                       vec![9, 6, 5, 4],
                   ]);
    }

    #[test]
    fn test_score() {
        let input = parse(
            "...0...
             ...1...
             ...2...
             6543456
             7.....7
             8.....8
             9.....9");

        assert_eq!(score_trailhead(&input, 0, 3), 2);

        let input = parse(
            "..90..9
             ...1.98
             ...2..7
             6543456
             765.987
             876....
             987....");

        assert_eq!(score_trailhead(&input, 0, 3), 4);

        let input = parse(
            "10..9..
             2...8..
             3...7..
             4567654
             ...8..3
             ...9..2
             .....01");

        assert_eq!(score_trailhead(&input, 0, 1), 1);
        assert_eq!(score_trailhead(&input, 6, 6), 2);
    }

    #[test]
    fn test_part1() {
        let input = parse(
            "89010123
             78121874
             87430965
             96549874
             45678903
             32019012
             01329801
             10456732");

        assert_eq!(part1(&input), 36);
    }

    #[test]
    fn test_rating() {
        let input = parse(
            ".....0.
             ..4321.
             ..5..2.
             ..6543.
             ..7..4.
             ..8765.
             ..9....");

        assert_eq!(rate_trailhead(&input, 0, 5), 3);

        let input = parse(
            "..90..9
             ...1.98
             ...2..7
             6543456
             765.987
             876....
             987....");

        assert_eq!(rate_trailhead(&input, 0, 3), 13);

        let input = parse(
            "012345
             123456
             234567
             345678
             4.6789
             56789.");

        assert_eq!(rate_trailhead(&input, 0, 0), 227);
    }

    #[test]
    fn test_part2() {
        let input = parse(
            "89010123
             78121874
             87430965
             96549874
             45678903
             32019012
             01329801
             10456732");

        assert_eq!(part2(&input), 81);
    }
}

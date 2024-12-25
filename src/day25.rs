#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashSet, HashMap};

type Input = Vec<Entry>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Entry {
    Key(u8, u8, u8, u8, u8),
    Lock(u8, u8, u8, u8, u8),
}
use Entry::*;

fn parse_one(lines: &[&str]) -> Entry {
    // let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();

    // let (mut result, is_key) = if lines[0] == "#####" { (Key(0,0,0,0,0), true) } else { (Lock(0,0,0,0,0), false) };
    let mut is_key = lines[0] == "#####";
    let mut result = if is_key { (0,0,0,0,0) } else { (5,5,5,5,5) };

    for line in lines[1..].iter() {
        let c = line.chars().nth(0).unwrap();
        result.0 += if c == '#' && is_key { 1 } else if c != '#' && !is_key { -1 } else { 0 };

        let c = line.chars().nth(1).unwrap();
        result.1 += if c == '#' && is_key { 1 } else if c != '#' && !is_key { -1 } else { 0 };

        let c = line.chars().nth(2).unwrap();
        result.2 += if c == '#' && is_key { 1 } else if c != '#' && !is_key { -1 } else { 0 };

        let c = line.chars().nth(3).unwrap();
        result.3 += if c == '#' && is_key { 1 } else if c != '#' && !is_key { -1 } else { 0 };

        let c = line.chars().nth(4).unwrap();
        result.4 += if c == '#' && is_key { 1 } else if c != '#' && !is_key { -1 } else { 0 };
    }

    let (a,b,c,d,e) = result;
    if is_key {
        Key (a as u8, b as u8, c as u8, d as u8,e as u8)
    } else {
        Lock(a as u8, b as u8, c as u8, d as u8, e as u8)
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Input {
    let v: Vec<_> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut result = Input::new();
    for c in v.chunks(7) {
        result.push(parse_one(c));
    }

    result
}

fn fit(key: &Entry, lock: &Entry) -> bool {
    if let Key(k1,k2,k3,k4,k5) = key {
        if let Lock(l1,l2,l3,l4,l5) = lock {
            if k1 + l1 <= 5 &&
                k2 + l2 <= 5 &&
                k3 + l3 <= 5 &&
                k4 + l4 <= 5 &&
                k5 + l5 <= 5
            {
                return true;
            }
        }
    }
    false
}

#[aoc(day25, part1)]
fn part1(input: &Input) -> u32 {
    let mut result = 0;

    // TODO: Very ugly implementation, maybe i can think of a better one.
    for key in input {
        for lock in input {
            if fit(key, lock) {
                result += 1
            }
        }
    }
    
    result
}

#[aoc(day25, part2)]
fn part2(input: &Input) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        parse(
            "#####
             .####
             .####
             .####
             .#.#.
             .#...
             .....
             
             #####
             ##.##
             .#.##
             ...##
             ...#.
             ...#.
             .....
             
             .....
             #....
             #....
             #...#
             #.#.#
             #.###
             #####
             
             .....
             .....
             #.#..
             ###..
             ###.#
             ###.#
             #####
             
             .....
             .....
             .....
             #....
             #.#..
             #.#.#
             #####"
        )
    }

    #[test]
    fn test_parse_one() {
        let key1 = parse_one(
            &["#####",
              ".####",
              ".####",
              ".####",
              ".#.#.",
              ".#...",
              "....."]
        );
    
        assert_eq!(key1, Key(0, 5, 3, 4, 3));
        
    
        let lock1 = parse_one(
            &[".....",
              "#....",
              "#....",
              "#...#",
              "#.#.#",
              "#.###",
              "#####"]
        );
        assert_eq!(lock1, Lock(5, 0, 2, 1, 3));
    }

    #[test]
    fn test_parse() {
        let input = sample_input();
        println!("{input:?}");
        assert_eq!(input.len(), 5);
        assert_eq!(input[0], Key(0,5,3,4,3));
        assert_eq!(input[1], Key(1,2,0,5,3));
        assert_eq!(input[2], Lock(5,0,2,1,3));
        assert_eq!(input[3], Lock(4,3,4,0,2));
        assert_eq!(input[4], Lock(3,0,2,0,1));
    }

    #[test]
    fn test_part1() {
        let input = sample_input();
        let result = part1(&input);
        assert_eq!(result, 3);
    }
}

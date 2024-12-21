#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashMap};
use memoize::memoize;

type Input = Vec<Vec<char>>;

#[derive(Copy, Clone, Eq, Debug, PartialEq, Hash)]
enum NumpadType {
    TypeA, TypeB
}

use NumpadType::*;

/*

Type A
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
PANIC! (3, 0)

*/

fn key_a(key: char) -> (usize, usize) {
    match key {
        '9' => (0, 2),
        '8' => (0, 1),
        '7' => (0, 0),
        '6' => (1, 2),
        '5' => (1, 1),
        '4' => (1, 0),
        '3' => (2, 2),
        '2' => (2, 1),
        '1' => (2, 0),
        '0' => (3, 1),
        'A' => (3, 2),
        _   => panic!("{key}")
    }
}

fn key_pos_a(pos: (usize, usize)) -> char {
    match pos {
        (0, 2) => '9',
        (0, 1) => '8',
        (0, 0) => '7',
        (1, 2) => '6',
        (1, 1) => '5',
        (1, 0) => '4',
        (2, 2) => '3',
        (2, 1) => '2',
        (2, 0) => '1',
        (3, 1) => '0',
        (3, 2) => 'A',
        _   => panic!("{pos:?}")
    }
}


/*
Type B
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
PANIC! (0, 0)
*/

fn key_b(key: char) -> (usize, usize) {
    match key {
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        'A' => (0, 2),
        _   => panic!("{key}"),
    }
}

fn key_pos_b(pos: (usize, usize)) -> char {
    match pos {
        (0, 1) => '^',
        (1, 0) => '<',
        (1, 1) => 'v',
        (1, 2) => '>',
        (0, 2) => 'A',
        _   => panic!("{pos:?}"),
    }
}

fn key_pos(numpad: NumpadType, pos: (usize, usize)) -> char {
    match numpad {
        TypeA => key_pos_a(pos),
        TypeB => key_pos_b(pos),
    }
}

fn key(numpad: NumpadType, key: char) -> (usize, usize) {
    match numpad {
        TypeA => key_a(key),
        TypeB => key_b(key),
    }
}

fn next_moves_a(key: char, to_key: char) -> Vec<(char, (usize, usize))> {
    let mut result = Vec::<(char, (usize, usize))>::new();

    let (cur_i, cur_j) = key_a(key);
    let (to_i, to_j) = key_a(to_key);

    // ^ < > v
    if to_j < cur_j && (cur_i != 3 || cur_j != 1) {
        result.push(('<', (cur_i, cur_j - 1)));
    }

    if to_i < cur_i {
        result.push(('^', (cur_i - 1, cur_j)));
    }

    if to_j > cur_j {
        result.push(('>', (cur_i, cur_j + 1)));
    }

    if to_i > cur_i && (cur_i != 2 || cur_j != 0) {
        result.push(('v', (cur_i + 1, cur_j)));
    }

    result
}

fn next_moves_b(key: char, to_key: char) -> Vec<(char, (usize, usize))> {
    let mut result = Vec::<(char, (usize, usize))>::new();

    let (cur_i, cur_j) = key_b(key);
    let (to_i, to_j) = key_b(to_key);

    // > v < ^
    if to_j > cur_j {
        result.push(('>', (cur_i, cur_j + 1)));
    }

    if to_i > cur_i {
        result.push(('v', (cur_i + 1, cur_j)));
    }

    if to_j < cur_j && (cur_i != 0 || cur_j != 1) {
        result.push(('<', (cur_i, cur_j - 1)));
    }

    if to_i < cur_i && (cur_i != 1 || cur_j != 0){
        result.push(('^', (cur_i - 1, cur_j)));
    }

    result
}

fn next_moves(numpad: NumpadType, key: char, to_key: char) -> Vec<(char, (usize, usize))> {
    match numpad {
        TypeA => next_moves_a(key, to_key),
        TypeB => next_moves_b(key, to_key),
    }
}

#[memoize]
fn move_to(numpad: NumpadType, from_key: char, to_key: char) -> Vec<Vec<char>> {
    if from_key == to_key {
        return vec![vec!['A']];
    }
    
    let mut cur = from_key;
    let mut result = Vec::<Vec<char>>::new();

    let moves = next_moves(numpad, cur, to_key);

    for (car, pos) in moves {

        let mut moves = move_to(numpad, key_pos(numpad, pos), to_key);

        for v in moves {
            let mut k = Vec::<char>::from([car]);
            v.iter().for_each(|it| k.push(*it));
            result.push(k);
        }
    }
    
    result
}

fn score_complexity(keys: Vec<char>, levels: u32) -> u64 {
    let num: u64 = String::from_iter(&keys[.. keys.len() - 1]).parse().unwrap();
    min_seq_length(TypeA, keys, levels + 1) * num
}

fn evaluate(numpad: NumpadType, seq: Vec<char>) -> Vec<char> {
    let mut cur = key(numpad, 'A');
    let mut result = Vec::<char>::new();

    for c in seq {
        match c {
            '<' => cur.1 = cur.1 - 1,
            '>' => cur.1 = cur.1 + 1,
            '^' => cur.0 = cur.0 - 1,
            'v' => cur.0 = cur.0 +1,
            'A' => result.push(key_pos(numpad, cur)),
            _   => panic!("{c} not valid"),
        }
    }
    result
}

#[memoize]
fn min_seq_length(numpad: NumpadType, keys: Vec<char>, level: u32) -> u64 {
    if level == 0 {
        return 1;
    }
    
    let mut prev_key = 'A';
    let mut result = 0;

    for key in &keys {
        let keys = move_to(numpad, prev_key, *key);

        let min_seq = keys
            .iter()
            .map(|keys| min_seq_length(TypeB, keys.clone(), level - 1))
            .min()
            .unwrap();

        result += min_seq;
        prev_key = *key;
    }

    result
}


#[aoc_generator(day21)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

#[aoc(day21, part1)]
fn part1(input: &Input) -> u64 {
    let mut out: u64 = 0;
    for i in input {
        out += score_complexity(i.clone(), 3);
    }
    out
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> u64 {
    let mut out: u64 = 0;
    for i in input {
        out += score_complexity(i.clone(), 26);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = parse(
            "029A
             980A
             179A
             456A
             379A"
        );
        assert_eq!(
            input,
            vec![
                vec!['0','2','9','A'],
                vec!['9','8','0','A'],
                vec!['1','7','9','A'],
                vec!['4','5','6','A'],
                vec!['3','7','9','A'],
            ])
    }

    #[test]
    fn test_move_to() {
        assert_eq!(move_to(TypeA, 'A', '9'), vec![vec!['^', '^', '^', 'A']]);
        assert_eq!(move_to(TypeA, 'A', '0'), vec![vec!['<', 'A']]);
        assert_eq!(move_to(TypeA, 'A', '1'), vec![vec!['<', '^', '<', 'A'], vec!['^', '<', '<', 'A']]);
        assert_eq!(move_to(TypeA, '0', '1'), vec![vec!['^', '<', 'A']]);
        
        assert_eq!(move_to(TypeA, '9', 'A'), vec![vec!['v', 'v', 'v', 'A']]);
        assert_eq!(move_to(TypeA, '0', 'A'), vec![vec!['>', 'A']]);
        assert_eq!(move_to(TypeA, '1', 'A'), vec![vec!['>', '>', 'v', 'A'], vec!['>', 'v', '>', 'A']]);
        assert_eq!(move_to(TypeA, '1', '0'), vec![vec!['>', 'v', 'A']]);
        
        assert_eq!(move_to(TypeB, 'A', '<'), vec![vec!['v', '<', '<', 'A'], vec!['<', 'v', '<', 'A']]);
        assert_eq!(move_to(TypeB, 'A', '^'), vec![vec!['<', 'A']]);
        assert_eq!(move_to(TypeB, 'A', 'v'), vec![vec!['v', '<', 'A'], vec!['<', 'v', 'A']]);
        assert_eq!(move_to(TypeB, 'A', '>'), vec![vec!['v', 'A']]);
        
        assert_eq!(move_to(TypeB, '<', 'A'), vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A']]);
        assert_eq!(move_to(TypeB, '^', 'A'), vec![vec!['>', 'A']]);
        assert_eq!(move_to(TypeB, 'v', 'A'), vec![vec!['>', '^', 'A'], vec!['^', '>', 'A']]);
        assert_eq!(move_to(TypeB, '>', 'A'), vec![vec!['^', 'A']]);
    }

    #[test]
    fn test_min_seq_length() {
        assert_eq!(min_seq_length(TypeA, "029A".chars().collect(), 4), 68);
        assert_eq!(min_seq_length(TypeA, "980A".chars().collect(), 4), 60);
        assert_eq!(min_seq_length(TypeA, "179A".chars().collect(), 4), 68);
        assert_eq!(min_seq_length(TypeA, "456A".chars().collect(), 4), 64);
        assert_eq!(min_seq_length(TypeA, "379A".chars().collect(), 4), 64);
    }
   
}



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

fn join(a: Vec<Vec<char>>, b: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if a.is_empty() {
        return b.clone();
    }

    if b.is_empty() {
        return a.clone();
    }

    let mut result = Vec::<Vec<char>>::new();
    for i in 0 .. a.len() {
        for j in 0 .. b.len() {
            let mut s = a[i].clone();
            s.append(&mut b[j].clone());
            result.push(s);
        }
    }
    result
}

fn generate_keys(numpad: NumpadType, keys: Vec<char>, level: u32) -> Vec<Vec<char>> {
    let mut result = Vec::<Vec<char>>::from([]);
    let mut prev_key = 'A';
    
    for k in keys {
        let mut keys = move_to(numpad, prev_key, k);
        result = join(result, keys);
        prev_key = k;
    }

    if level > 1 {
        let mut rr = Vec::<Vec<char>>::new();
        
        for r in result {
            let mut rl = generate_keys(TypeB, r, level - 1);
            rr.append(&mut rl);
        }
        return rr;
    }

    // println!("{}", String::from_iter(&result));
    result
}

fn seq_length(sequence: Vec<char>) -> u64 {
    let r = generate_keys(TypeA, sequence, 3);
    let m = r.iter().min_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap();
    m.len() as u64
}

fn score_complexity(sequence: Vec<char>) -> u64 {
    let num: u64 = String::from_iter(&sequence[.. sequence.len() - 1]).parse().unwrap();
    seq_length(sequence) * num
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
        out += score_complexity(i.clone())
    }
    out
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> u64 {
    todo!()
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
    fn test_join_lists() {

        let a = vec![vec![]];
        let b = vec![vec!['a','a']];
        assert_eq!(join(a, b), vec![vec!['a','a']]);

        let a = vec![vec!['a','a']];
        let b = vec![vec!['b','b']];
        assert_eq!(join(a, b), vec![
            vec!['a','a','b','b']
        ]);
        
        let a = vec![vec!['a','a']];
        let b = vec![vec!['b','b'], vec!['c','c']];
        assert_eq!(join(a, b), vec![
            vec!['a','a','b','b'],
            vec!['a','a','c','c'],
        ]);

        let a = vec![vec!['a','a'], vec!['b','b']];
        let b = vec![vec!['c','c'], vec!['d','d']];
        assert_eq!(join(a, b), vec![
            vec!['a','a','c','c'],
            vec!['a','a','d','d'],
            vec!['b','b','c','c'],
            vec!['b','b','d','d'],
        ]);
    }

    #[test]
    fn test_generate_keys() {
        assert_eq!(
            generate_keys(TypeA, "029A".chars().collect(), 1),
            vec![vec!['<', 'A', '^', 'A', '^', '^', '>', 'A', 'v', 'v', 'v', 'A'],
                 vec!['<', 'A', '^', 'A', '^', '>', '^', 'A', 'v', 'v', 'v', 'A'],
                 vec!['<', 'A', '^', 'A', '>', '^', '^', 'A', 'v', 'v', 'v', 'A']]);

        let r = generate_keys(TypeA, "029A".chars().collect(), 3);
        let m = r.iter().min_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap();
        assert_eq!(m.len(), 68);

        let r = generate_keys(TypeA, "980A".chars().collect(), 3);
        let m = r.iter().min_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap();
        assert_eq!(m.len(), 60);

        let r = generate_keys(TypeA, "179A".chars().collect(), 3);
        let m = r.iter().min_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap();
        assert_eq!(m.len(), 68);

        let r = generate_keys(TypeA, "456A".chars().collect(), 3);
        let m = r.iter().min_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap();
        assert_eq!(m.len(), 64);

        let r = generate_keys(TypeA, "379A".chars().collect(), 3);
        let m = r.iter().min_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap();
        assert_eq!(m.len(), 64);
    }

    
    /*
    #[test]
    fn test_sequence() {
        let input = vec!['3','7','9','A'];
        assert_eq!(
            evaluate(TypeA, generate_keys(TypeA, 'A', input.clone())),
            input
        );

        let input = generate_keys(TypeA, 'A', vec!['3','7','9','A']);
        assert_eq!(
            evaluate(TypeB, generate_keys(TypeB, 'A', input.clone())),
            input
        );

        let input = generate_keys(TypeB, 'A', generate_keys(TypeB, 'A', generate_keys(TypeA, 'A', vec!['3','7','9','A'])));
        assert_eq!(
            evaluate(TypeB, generate_keys(TypeB, 'A', input.clone())),
            input
        );

        //assert_eq!(
        //    generate_keys(TypeB, 'A', vec!['<','A','^','A','^','^','>','A','v','v','v','A'],).len(),
        //    28
        //);
        //assert_eq!(
        //    generate_keys(TypeB, 'A', vec!['v','<','<','A','>','>','^','A','<','A','>','A','v','A','<','^','A','A','>','A','<','v','A','A','A','>','^','A'],).len(),
        //    68
        //);

        println!("=====");

        // println!("{}", String::from_iter(evaluate(TypeB, "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".chars().collect())));
        // println!("{}", String::from_iter(evaluate(TypeB, "<A>Av<<AA>^AA>AvAA^A<vAAA>^A".chars().collect())));
        // println!("{}", String::from_iter(evaluate(TypeA, "^A<<^^A>>AvvvA".chars().collect())));

        println!("{}", String::from_iter(generate_keys(TypeA, 'A', "379A".chars().collect())));
        println!("{}", String::from_iter(generate_keys(TypeB, 'A', "^A<<^^A>>AvvvA".chars().collect())));
        println!("{}", String::from_iter(generate_keys(TypeB, 'A', "<A>Av<<AA>^AA>AvAA^A<vAAA>^A".chars().collect())));

        println!("=====");

        // assert_eq!(
        //     generate_keys(TypeB, 'A', generate_keys(TypeB, 'A', generate_keys(TypeA, 'A', vec!['3','7','9','A'],))).len(),
        //     64);


    }

    */

    #[test]
    fn test_score() {
        assert_eq!(seq_length(vec!['0','2','9','A']), 68);
        assert_eq!(seq_length(vec!['9','8','0','A']), 60);
        assert_eq!(seq_length(vec!['1','7','9','A']), 68);
        assert_eq!(seq_length(vec!['4','5','6','A']), 64);
        assert_eq!(seq_length(vec!['3','7','9','A']), 64);
    }
}



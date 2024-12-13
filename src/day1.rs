#![allow(unused_variables, unused_mut)]

use std::collections::HashMap;

type Input = (Vec<u32>, Vec<u32>);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    
    input
        .lines()
        .for_each(|l| {
            let mut iter = l.trim().split_whitespace();
    
            let a: u32 = iter.next().unwrap().parse().unwrap();
            let b: u32 = iter.next().unwrap().parse().unwrap();
    
            v1.push(a);
            v2.push(b);
        });

    (v1, v2)
}

#[aoc(day1, part1)]
pub fn part1((v1, v2): &Input) -> u32 {
    let mut v1 = v1.clone();
    let mut v2 = v2.clone();

    v1.sort();
    v2.sort();

    let mut iter_v1 = v1.iter();
    let mut iter_v2 = v2.iter();

    let mut acc = 0;

    while let Some(elem1) = iter_v1.next() {
        let elem2 = iter_v2.next().unwrap_or(&0);
        let result = elem2.abs_diff(*elem1);
        acc += result;
    }

    return acc;
}

#[aoc(day1, part2)]
pub fn part2((v1, v2): &Input) -> u32 {

    let mut iter_v1 = v1.iter();
    let mut iter_v2 = v2.iter();
    let mut count_map = HashMap::<u32, u32>::new();

    // Constructs the count map
    while let Some(elem) = iter_v2.next() {
        if count_map.contains_key(elem) {
            let mut cval = count_map.get_mut(elem).unwrap();
            *cval += 1;
        } else {
            count_map.insert(*elem, 1);
        }
    }

    let mut acc = 0;
    while let Some(elem) = iter_v1.next() {
        let val = match count_map.get(elem) {
            None => 0,
            Some(val) => *val
        };
        acc += val * elem;
    }

    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator() {
        let i = input_generator("100   20\n10  200\n20    5");
        assert_eq!(i, (vec![100, 10, 20], vec![20, 200, 5]));
    }
    
    #[test]
    fn p1_case1() {
        let v1 = vec![10, 0];
        let v2 = vec![0, 10];
        
        assert_eq!(0, part1(&(v1, v2)));
    }

    #[test]
    fn p1_case2() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];
        
        assert_eq!(11, part1(&(v1, v2)));
    }

    #[test]
    fn p2_case1() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];
        
        assert_eq!(31, part2(&(v1, v2)));
    }
}

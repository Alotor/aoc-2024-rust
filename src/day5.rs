#![allow(unused_variables, unused_mut, dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

type Input = (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>);


#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut rules = HashMap::new();
    let mut lists = Vec::new();

    let mut lines_it = input.lines().map(|l| l.trim());

    while let Some(line) = lines_it.next() {
        if line == "" {
            break;
        }
        
        let sp: Vec<&str> = line.split("|").collect();
        let n: u32 = sp[0].parse().unwrap();
        let m: u32 = sp[1].parse().unwrap();

        if rules.contains_key(&n) {
            let mut cval: &mut HashSet<_> = rules.get_mut(&n).unwrap();
            cval.insert(m);
        } else {
            let mut cval = HashSet::new();
            cval.insert(m);
            rules.insert(n, cval);
        }
    }

    while let Some(line) = lines_it.next() {
        lists.push(
            line.split(",").map(|n| n.parse::<u32>().unwrap()).collect()
        );
    }
    (rules, lists)
}

fn check(rules: &HashMap<u32, HashSet<u32>>, processed: &HashSet<u32>, elem: &u32) -> bool {
    processed.iter().all(|e| {
        if let Some(rule) = rules.get(&e) {
            !rule.contains(elem)
        } else {
            true
        }
    })
}

fn sort(rules: &HashMap<u32, HashSet<u32>>, elems: &mut Vec<u32>) {
    elems.sort_by(|a, b| {
        if let Some(rule) = rules.get(a) {
            if rule.contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Equal
        }
    });
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    let mut result = 0;
    let rules = &input.0;

    input.1.iter().for_each(|elem| {
        let mut processed = HashSet::<u32>::new();
        let mut valid = true;
        
        for e in elem.iter().rev() {
            if !check(rules, &processed, e) {
                valid = false;
                break;
            }
            processed.insert(*e);
        }

        if valid {
            result += elem[elem.len() / 2];
        }
    });

    return result
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    let mut result = 0;
    let rules = &input.0;

    input.1.iter().for_each(|elem| {
        let mut processed = HashSet::<u32>::new();
        let mut valid = true;
        
        for e in elem.iter().rev() {
            if !check(rules, &processed, e) {
                valid = false;
                break;
            }
            processed.insert(*e);
        }

        if !valid {
            let mut elem = elem.clone();
            sort(rules, &mut elem);
            result += elem[elem.len() / 2];
        }
    });

    return result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let input = parse(
            "47|53
             97|13
             97|61
             97|47
             75|29
             61|13
             75|53
             29|13
             97|29
             53|29
             61|53
             97|53
             61|29
             47|13
             75|47
             97|75
             47|61
             75|61
             47|29
             75|13
             53|13
             
             75,47,61,53,29
             97,61,53,29,13
             75,29,13
             75,97,47,61,53
             61,13,29
             97,13,75,29,47");

        let set75 = input.0.get(&75).unwrap();
        assert!(set75.contains(&29));
        assert!(set75.contains(&53));
        assert!(set75.contains(&47));
        assert!(set75.contains(&61));
        assert!(set75.contains(&13));

        let set53 = input.0.get(&53).unwrap();
        assert!(set53.contains(&29));
        assert!(set53.contains(&13));

        let set97 = input.0.get(&97).unwrap();
        assert!(set97.contains(&13));
        assert!(set97.contains(&61));
        assert!(set97.contains(&47));
        assert!(set97.contains(&29));
        assert!(set97.contains(&53));
        assert!(set97.contains(&75));

        assert_eq!(input.1[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(input.1[5], vec![97, 13, 75, 29, 47]);
    }

    #[test]
    fn test_check() {
        let input = parse(
            "47|53
             97|13
             97|61
             97|47
             75|29
             61|13
             75|53
             29|13
             97|29
             53|29
             61|53
             97|53
             61|29
             47|13
             75|47
             97|75
             47|61
             75|61
             47|29
             75|13
             53|13
             
             75,47,61,53,29
             97,61,53,29,13
             75,29,13
             75,97,47,61,53
             61,13,29
             97,13,75,29,47");


        assert!(check(&input.0, &vec![29, 53].into_iter().collect(), &75));
        assert!(!check(&input.0, &vec![97,47,61,53].into_iter().collect(), &75));
    }


    
    #[test]
    fn test_part1() {
        let input = parse(
            "47|53
             97|13
             97|61
             97|47
             75|29
             61|13
             75|53
             29|13
             97|29
             53|29
             61|53
             97|53
             61|29
             47|13
             75|47
             97|75
             47|61
             75|61
             47|29
             75|13
             53|13

             75,47,61,53,29
             97,61,53,29,13
             75,29,13
             75,97,47,61,53
             61,13,29
             97,13,75,29,47");

        assert_eq!(part1(&input), 143);
    }

    #[test]
    fn test_part2() {
        let input = parse(
            "47|53
             97|13
             97|61
             97|47
             75|29
             61|13
             75|53
             29|13
             97|29
             53|29
             61|53
             97|53
             61|29
             47|13
             75|47
             97|75
             47|61
             75|61
             47|29
             75|13
             53|13

             75,47,61,53,29
             97,61,53,29,13
             75,29,13
             75,97,47,61,53
             61,13,29
             97,13,75,29,47");

        assert_eq!(part2(&input), 123);
    }
}

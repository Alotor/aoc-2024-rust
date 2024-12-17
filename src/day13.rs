#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use regex::Regex;
use memoize::memoize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Entry {
    ax: u32,
    ay: u32,
    bx: u32,
    by: u32,
    price_x: u32,
    price_y: u32,
}

type Input = Vec<Entry>;

#[aoc_generator(day13)]
fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    lines
        .chunks(3)
        .map(|lines| {
            let button_re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
            let price_re = Regex::new(r"X\=(\d+), Y\=(\d+)").unwrap();

            let ac = button_re.captures(lines[0]).unwrap();
            let bc = button_re.captures(lines[1]).unwrap();
            let pr = price_re.captures(lines[2]).unwrap();
            
            Entry {
                ax: ac[1].parse::<u32>().unwrap(),
                ay: ac[2].parse::<u32>().unwrap(),
                bx: bc[1].parse::<u32>().unwrap(),
                by: bc[2].parse::<u32>().unwrap(),
                price_x: pr[1].parse::<u32>().unwrap(),
                price_y: pr[2].parse::<u32>().unwrap(),
            }})
        .collect()
}

fn min_tokens(entry: Entry) -> Option<u32> {
    #[memoize]
    fn min_tokens_rec(entry: Entry, tok_a: u32, tok_b: u32, px: u32, py: u32) -> Option<u32> {
        if px == entry.price_x && py == entry.price_y {
            Some(tok_a * 3 + tok_b)
        } else if px > entry.price_x || py > entry.price_y {
            None
        } else {
            let ra = min_tokens_rec(entry, tok_a + 1, tok_b, px + entry.ax, py + entry.ay);
            let rb = min_tokens_rec(entry, tok_a, tok_b + 1, px + entry.bx, py + entry.by);

            match (ra, rb) {
                (None, None) => None,
                (Some(p), None) | (None, Some(p)) => Some(p),
                (Some(a), Some(b)) => {
                    if a < b {
                        Some(a)
                    } else {
                        Some(b)
                    }
                }
            }
        }
    }
    min_tokens_rec(entry, 0, 0, 0, 0)
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|entry| min_tokens(*entry).unwrap_or(0))
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = parse(
            "Button A: X+94, Y+34
             Button B: X+22, Y+67
             Prize: X=8400, Y=5400
             
             Button A: X+26, Y+66
             Button B: X+67, Y+21
             Prize: X=12748, Y=12176
             
             Button A: X+17, Y+86
             Button B: X+84, Y+37
             Prize: X=7870, Y=6450
             
             Button A: X+69, Y+23
             Button B: X+27, Y+71
             Prize: X=18641, Y=10279");

        assert_eq!(input, vec![
            Entry { ax: 94, ay: 34, bx: 22, by: 67, price_x: 8400, price_y: 5400 },
            Entry { ax: 26, ay: 66, bx: 67, by: 21, price_x: 12748, price_y: 12176 },
            Entry { ax: 17, ay: 86, bx: 84, by: 37, price_x: 7870, price_y: 6450 },
            Entry { ax: 69, ay: 23, bx: 27, by: 71, price_x: 18641, price_y: 10279 },
        ]);
    }

    #[test]
    fn test_min_tokens() {
        
        assert_eq!(min_tokens(
            Entry { ax: 94, ay: 34, bx: 22, by: 67, price_x: 8400, price_y: 5400 }),
            Some(280));
        
        assert_eq!(min_tokens(
            Entry { ax: 26, ay: 66, bx: 67, by: 21, price_x: 12748, price_y: 12176 }),
            None);
        
        assert_eq!(min_tokens(
            Entry { ax: 17, ay: 86, bx: 84, by: 37, price_x: 7870, price_y: 6450 }),
            Some(200));
        
        assert_eq!(min_tokens(
            Entry { ax: 69, ay: 23, bx: 27, by: 71, price_x: 18641, price_y: 10279 }),
            None);
    }

    #[test]
    fn test_part1() {
        let input = parse(
            "Button A: X+94, Y+34
             Button B: X+22, Y+67
             Prize: X=8400, Y=5400
             
             Button A: X+26, Y+66
             Button B: X+67, Y+21
             Prize: X=12748, Y=12176
             
             Button A: X+17, Y+86
             Button B: X+84, Y+37
             Prize: X=7870, Y=6450
             
             Button A: X+69, Y+23
             Button B: X+27, Y+71
             Prize: X=18641, Y=10279");

        assert_eq!(part1(&input), 480);
    }
}

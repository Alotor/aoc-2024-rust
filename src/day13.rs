#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use regex::Regex;
use memoize::memoize;
use nalgebra::*;

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

// My naive solution using recursion. Left here for historic purposes :P
/*
fn min_tokens_long(entry: Entry, _: f64) -> Option<u32> {
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
*/

fn try_u64(num: f64) -> Option<u64> {
    if num.fract() <= 0.0001  {
        Some(num as u64)
    } else if num.fract() >= 0.9999 {
        Some(num as u64 + 1)
    } else {
        None
    }
}

fn min_tokens(entry: Entry, modifier: f64) -> Option<u64> {
    let m = Matrix2::<f64>::new(
        entry.ax.into(),
        entry.ay.into(),
        entry.bx.into(),
        entry.by.into());

    let r = Matrix1x2::<f64>::new(
        modifier + entry.price_x as f64,
        modifier + entry.price_y as f64);

    if let Some(inverse) = m.try_inverse() {
        let sol = r * inverse;
        let sa = sol[0];
        let sb = sol[1];

        if let (Some(a), Some(b)) = (try_u64(sa), try_u64(sb)) {
            Some(3 * a + b)
        } else {
            None
        }
    } else {
        None
    }
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> u64 {
    input
        .iter()
        .map(|entry| min_tokens(*entry, 0.0).unwrap_or(0))
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> u64 {
    input
        .iter()
        .map(|entry| min_tokens(*entry, 1e13).unwrap_or(0))
        .sum()
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
        assert_eq!(min_tokens(Entry { ax: 94, ay: 34, bx: 22, by: 67, price_x: 8400, price_y: 5400 }, 0.0), Some(280));
        assert_eq!(min_tokens(Entry { ax: 26, ay: 66, bx: 67, by: 21, price_x: 12748, price_y: 12176 }, 0.0), None);
        assert_eq!(min_tokens(Entry { ax: 17, ay: 86, bx: 84, by: 37, price_x: 7870, price_y: 6450 }, 0.0), Some(200));
        assert_eq!(min_tokens(Entry { ax: 69, ay: 23, bx: 27, by: 71, price_x: 18641, price_y: 10279 }, 0.0), None);

        // I got these cases by executing my naive solution
        assert_eq!(min_tokens(Entry { ax: 98, ay: 41, bx: 59, by: 88, price_x: 6029, price_y: 6258 }, 0.0), Some(137));
        assert_eq!(min_tokens(Entry { ax: 31, ay: 88, bx: 75, by: 36, price_x: 8434, price_y: 8728 }, 0.0), Some(278));
        assert_eq!(min_tokens(Entry { ax: 17, ay: 90, bx: 34, by: 11, price_x: 1496, price_y: 5047 }, 0.0), Some(179));
        assert_eq!(min_tokens(Entry { ax: 52, ay: 99, bx: 47, by: 15, price_x: 8241, price_y: 8316 }, 0.0), Some(306));
        assert_eq!(min_tokens(Entry { ax: 34, ay: 35, bx: 12, by: 96, price_x: 3226, price_y: 7085 }, 0.0), Some(282));
        assert_eq!(min_tokens(Entry { ax: 98, ay: 44, bx: 51, by: 74, price_x: 7348, price_y: 5752 }, 0.0), Some(198));
        assert_eq!(min_tokens(Entry { ax: 27, ay: 17, bx: 20, by: 89, price_x: 3108, price_y: 7229 }, 0.0), Some(261));
        assert_eq!(min_tokens(Entry { ax: 98, ay: 56, bx: 37, by: 66, price_x: 4967, price_y: 3870 }, 0.0), Some(149));
        assert_eq!(min_tokens(Entry { ax: 36, ay: 55, bx: 63, by: 12, price_x: 2988, price_y: 4228 }, 0.0), Some(232));
        assert_eq!(min_tokens(Entry { ax: 87, ay: 17, bx: 28, by: 54, price_x: 8764, price_y: 4236 }, 0.0), Some(304));
        assert_eq!(min_tokens(Entry { ax: 43, ay: 45, bx: 69, by: 17, price_x: 4224, price_y: 1660 }, 0.0), Some(104));
        assert_eq!(min_tokens(Entry { ax: 27, ay: 66, bx: 53, by: 29, price_x: 4020, price_y: 4095 }, 0.0), Some(168));
        assert_eq!(min_tokens(Entry { ax: 80, ay: 86, bx: 18, by: 94, price_x: 4434, price_y: 7230 }, 0.0), Some(177));
        assert_eq!(min_tokens(Entry { ax: 78, ay: 67, bx: 16, by: 65, price_x: 6426, price_y: 7365 }, 0.0), Some(261));
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

    #[test]
    fn test_part2() {
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
        
        assert_eq!(part2(&input), 875318608908);
    }
}

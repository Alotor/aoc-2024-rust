#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashMap};

type Input = Vec<u64>;

#[aoc_generator(day22)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

fn p1(secret: u64) -> u64 {
    prune(mix(secret * 64, secret))
}

fn p2(secret: u64) -> u64 {
    prune(mix(secret / 32, secret))
}

fn p3(secret: u64) -> u64 {
    prune(mix(secret * 2048, secret))
}

fn next(secret: u64) -> u64 {
    p3(p2(p1(secret)))
}

fn generate(secret: u64, size: u32) -> u64 {
    let mut cur = secret;
    for _ in 0 .. size {
        cur = p3(p2(p1(cur)));
    }
    cur
}

fn prices(secret: u64, size: u32) -> Vec<i64> {
    let mut result = Vec::new();
    let mut cur = secret;
    result.push(cur  as i64 % 10);
    for _ in 0 .. size {
        cur = p3(p2(p1(cur)));
        result.push(cur  as i64 % 10);
    }
    result
}

fn price_deltas(prices: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::<i64>::new();
    let mut prev = prices[0];
    for cur in prices {
        result.push(cur - prev);
        prev = *cur;
    }
    result
}

fn price_commands(prices: Vec<i64>) -> HashMap<Vec<i64>, i64> {
    let deltas = price_deltas(&prices);
    let mut result = HashMap::new();

    let mut tar_i = 4;
    deltas[1..].windows(4).for_each(|w| {
        let command = Vec::from(w);
        if !result.contains_key(&command) {
            result.insert(Vec::from(w), prices[tar_i]);
        }
        tar_i += 1;
    });

    result
}

#[aoc(day22, part1)]
fn part1(input: &Input) -> u64 {
    input
        .iter()
        .map(|n| generate(*n, 2000))
        .sum()
}

// #[aoc(day22, part2)]
fn experiments(input: &Input) -> i64 {

    let mut commands = Vec::<HashMap<Vec<i64>, i64>>::new();

    println!("Generating...");
    for i in input {
        let comms = price_commands(prices(*i, 2000));
        commands.push(comms);
    }

    let command = vec![3, 1, 4, 1];
    let mut acc = 0;
    for i in 0 .. commands.len() {
        let entry = &commands[i];
        let cc = *entry.get(&command).unwrap_or(&0);
        acc += cc;
        if cc != 0 {
            println!("{i}: {cc}");
        }
        
    }

    println!(">> {acc}");
    

    /*
    let mut max_val = 0;
    let mut max_seq = &Vec::<i64>::new();

    for i in 0 .. commands.len() {
        println!("Check {} / {}. Max {}, {:?}",
                 i+1, commands.len(), max_val, max_seq);
        let entry = &commands[i];
        for (command, _) in entry.into_iter() {
            let mut acc = 0;
            for j in 0 .. commands.len() {
                let other = &commands[j];

                let current = *other.get(command).unwrap_or(&0);
                acc += current;
            }

            if acc > max_val {
                max_val = acc;
                max_seq = command;
            }
        }
    }

    println!("{max_seq:?} => {max_val}");
    max_val
     */
    0
}

#[aoc(day22, part2)]
fn part2(input: &Input) -> i64 {

    let mut commands = Vec::<HashMap<Vec<i64>, i64>>::new();

    println!("Generating...");
    for i in input {
        commands.push(price_commands(prices(*i, 2000)));
    }
    println!("Generating DONE");

    let mut max_val = 0;
    let mut max_seq = &Vec::<i64>::new();

    for i in 0 .. commands.len() {
        let entry = &commands[i];
        for (command, _) in entry.into_iter() {
            let mut acc = 0;
            for j in 0 .. commands.len() {
                let other = &commands[j];

                let current = *other.get(command).unwrap_or(&0);
                acc += current;
            }

            if acc > max_val {
                max_val = acc;
                max_seq = command;
            }
        }
        println!("Check {} / {}. Max {}, {:?}",
                 i+1, commands.len(), max_val, max_seq);
    }

    println!("{max_seq:?} => {max_val}");
    max_val
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = parse(
            "1\n10\n100\n2024"
        );
        assert_eq!(input, vec![1, 10, 100, 2024]);
    }

    #[test]
    fn test_next() {
        let nexts = [
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ];

        let mut cur = 123;
        for n in nexts {
            cur = next(cur);
            assert_eq!(cur, n);
        }
    }

    #[test]
    fn test_generate() {
        assert_eq!(generate(123, 10), 5908254);
        assert_eq!(generate(1, 2000), 8685429);
        assert_eq!(generate(10, 2000), 4700978);
        assert_eq!(generate(100, 2000), 15273692);
        assert_eq!(generate(2024, 2000), 8667524);
    }

    #[test]
    fn test_part1() {
        let input = parse(
            "1\n10\n100\n2024"
        );
        assert_eq!(part1(&input), 37327623);
    }

    #[test]
    fn test_prices() {
        assert_eq!(
            prices(123, 10),
            vec![
                3, 0, 6, 5, 4, 4, 6, 4, 4, 2, 4
            ]
        );
    }

    #[test]
    fn test_price_deltas() {
        assert_eq!(
            price_deltas(&prices(123, 10)),
            vec![
                0, -3, 6, -1, -1, 0, 2, -2, 0, -2, 2
            ]
        );
    }

    #[test]
    fn test_price_commands() {
        let commands = price_commands(
            vec![
                3, 0, 6, 5, 4, 4, 6, 4, 4, 2
            ]
        );
        assert_eq!(*commands.get(&vec![-3,  6, -1, -1]).unwrap(), 4);
        assert_eq!(*commands.get(&vec![ 6, -1, -1,  0]).unwrap(), 4);
        assert_eq!(*commands.get(&vec![-1, -1,  0,  2]).unwrap(), 6);
        assert_eq!(*commands.get(&vec![-1,  0,  2, -2]).unwrap(), 4);
        assert_eq!(*commands.get(&vec![ 0,  2, -2,  0]).unwrap(), 4);
        assert_eq!(*commands.get(&vec![ 2, -2,  0, -2]).unwrap(), 2);
    }

    #[test]
    fn test_sample_input() {
        let c1 = price_commands(prices(1, 2000));
        let c2 = price_commands(prices(2, 2000));
        let c3 = price_commands(prices(3, 2000));
        let c4 = price_commands(prices(2024, 2000));

        assert_eq!(*c1.get(&vec![-2, 1, -1, 3]).unwrap_or(&0), 7);
        assert_eq!(*c2.get(&vec![-2, 1, -1, 3]).unwrap_or(&0), 7);
        assert_eq!(*c3.get(&vec![-2, 1, -1, 3]).unwrap_or(&0), 0);
        assert_eq!(*c4.get(&vec![-2, 1, -1, 3]).unwrap_or(&0), 9);
    }

    #[test]
    fn test_all_commands() {
        let input = parse(
            "1\n2\n3\n2024"
        );

        assert_eq!(part2(&input), 23);
    }

    #[test]
    fn test_sample_1() {
        let input = parse(
            "2021\n5017\n19751"
        );

        let tmp = vec![1,2,3,4,5,6,7,8];
        tmp[1..].windows(4).for_each(|w|{
            println!("{w:?}");
        });


        assert_eq!(part2(&input), 27);
    }

    #[test]
    fn test_sample_2() {
        let input = parse(
            "5053\n10083\n11263"
        );

        assert_eq!(part2(&input), 27);
    }
}

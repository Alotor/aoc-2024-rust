#![allow(unused_variables, unused_mut, dead_code)]

use crate::utils::*;
use std::collections::HashSet;

type Input = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    N, S, W, E,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

fn find_start(input: &Input) -> (i32, i32) {
    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            let i = i as i32;
            let j = j as i32;
            if get2d(input, i, j) == &'^' {
                return (i, j)
            }
        }
    }
    unreachable!()
}

fn rotate(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
    }
}

fn sim_step(input: &Input, position: (i32, i32), dir: Dir) -> Option<((i32, i32), Dir)> {
    let (mut i, mut j) = position;

    match dir {
        Dir::N => i = i - 1,
        Dir::E => j = j + 1,
        Dir::S => i = i + 1,
        Dir::W => j = j - 1,
    };

    match *get2d(input, i, j) {
        '.' | '^' => Some(((i, j), dir)),
        '#' => Some((position, rotate(dir))),
        _ => None
    }
}

fn sim_step_2(input: &Input, position: (i32, i32), dir: Dir, obstacle: &(i32, i32)) -> Option<((i32, i32), Dir)> {
    let (mut i, mut j) = position;

    match dir {
        Dir::N => i = i - 1,
        Dir::E => j = j + 1,
        Dir::S => i = i + 1,
        Dir::W => j = j - 1,
    };

    if obstacle.0 == i && obstacle.1 == j {
        Some((position, rotate(dir)))
    } else {
        match *get2d(input, i, j) {
            '.' | '^' => Some(((i, j), dir)),
            '#' => Some((position, rotate(dir))),
            _ => None
        }
    }
}

fn print_sim(input: &Input, position: (i32, i32), processed: HashSet<(i32, i32)>) {
    println!("=======");

    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            let i = i as i32;
            let j = j as i32;
            let c = get2d(input, i, j);

            if (i, j) == position {
                print!("*");
            } else if processed.contains(&(i, j)) {
                print!("X");
            } else {
                print!("{c}");
            }
            
        }
        println!();
    }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> i32 {
    let mut position = find_start(input);
    let mut processed = HashSet::<(i32, i32)>::new();
    let mut dir = Dir::N;

    processed.insert(position);
    
    while let Some(((i, j), new_dir)) = sim_step(input, position, dir) {
        position = (i, j);
        dir = new_dir;

        processed.insert(position);
    }

    return processed.len() as i32;
}

fn check_loop(input: &Input, position: &(i32, i32), obstacle: &(i32, i32)) -> bool {
    let mut processed = HashSet::<((i32, i32), Dir)>::new();
    let mut dir = Dir::N;
    let mut position = *position;
    
    processed.insert((position, dir));
    
    while let Some(((i, j), new_dir)) = sim_step_2(input, position, dir, obstacle) {
        position = (i, j);
        dir = new_dir;

        if processed.contains(&(position, new_dir)) {
            // Loop found
            return true
        }
        
        processed.insert((position, new_dir));
    }

    return false;
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> i32 {
    let mut position = find_start(input);
    let mut loops = 0;

    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            let i = i as i32;
            let j = j as i32;

            if i == position.0 && j == position.1 {
                continue;
            }

            let is_loop = check_loop(input, &position, &(i, j));

            if is_loop {
                loops += 1
            }
        }
    }

    loops
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start() {
        let (i, j) = find_start(
            &parse("....#.....
                    .........#
                    ..........
                    ..#.......
                    .......#..
                    ..........
                    .#..^.....
                    ........#.
                    #.........
                    ......#..."));

        assert_eq!(i, 6);
        assert_eq!(j, 4);
    }

    #[test]
    fn test_parse() {
        let input = parse("....#.....
                           .........#
                           ..........
                           ..#.......
                           .......#..
                           ..........
                           .#..^.....
                           ........#.
                           #.........
                           ......#...");

        assert_eq!(get2d(&input, 0, 0), &'.');
        assert_eq!(get2d(&input, 0, 4), &'#');
        assert_eq!(get2d(&input, 6, 4), &'^');
        assert_eq!(get2d(&input, 10, 4), &' ');
        assert_eq!(get2d(&input, 6, 10), &' ');
    }

    #[test]
    fn test_part1() {
        let input = parse("....#.....
                           .........#
                           ..........
                           ..#.......
                           .......#..
                           ..........
                           .#..^.....
                           ........#.
                           #.........
                           ......#...");

        assert_eq!(part1(&input), 41);
    }

    #[test]
    fn test_part2() {
        let input = parse("....#.....
                           .........#
                           ..........
                           ..#.......
                           .......#..
                           ..........
                           .#..^.....
                           ........#.
                           #.........
                           ......#...");

        assert_eq!(part2(&input), 6);
    }
}

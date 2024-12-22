#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashMap, HashSet};
use colored::Colorize;

type Input = Vec<Vec<char>>;

fn find(map: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for i in 0 .. map.len() {
        for j in 0 .. map[i].len() {
            if map[i][j] == c {
                return (j, i);
            }
        }
    }
    panic!();
}

fn print_map(map: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) {
    let path_points = HashSet::<&(usize, usize)>::from_iter(path);
    for i in 0 .. map.len() {
        for j in 0 .. map[i].len() {
            if map[i][j] == 'S' || map[i][j] == 'E' {
                print!("{}", map[i][j].to_string().yellow());
            } else if path_points.contains(&(j, i)) {
                print!("{}", "O".red());
            } else {
                print!("{}", map[i][j].to_string().blue());
            }
        }
        println!();
    }
}

fn h(end: (usize, usize)) -> impl Fn((usize, usize)) -> u32 {
    move |(i, j)| {
        let i = i as f32;
        let j = j as f32;
        let a = end.0 as f32;
        let b = end.1 as f32;

        let res = ((i - a) * (i - a) + (j - b) * (j - b)).sqrt();
        res as u32    
    }
}

fn min(open: &HashSet::<(usize, usize)>, fscore: &HashMap::<(usize, usize), u32>) -> (usize, usize) {
    *open
        .iter()
        .min_by(|p1, p2| {
            let s1 = fscore.get(p1).unwrap_or(&u32::MAX);
            let s2 = fscore.get(p2).unwrap_or(&u32::MAX);
            s1.cmp(s2)
        })
        .unwrap()
}

fn neighbours(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<((usize, usize), u32)> {
    let mut result = Vec::new();
    if x > 0 && map[y][x-1] != '#' {
        result.push(((x - 1, y), 1));
    }
    if x < map[y].len() - 1 && map[y][x+1] != '#' {
        result.push(((x + 1, y), 1));
    }
    if y > 0 && map[y-1][x] != '#' {
        result.push(((x, y - 1), 1));
    }
    if y < map.len() - 1 && map[y+1][x] != '#' {
        result.push(((x, y + 1), 1));
    }

    result
}

fn build_path(from_node: &HashMap::<(usize, usize), (usize, usize)>, last: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::from([*last]);
    let mut current = last;

    while let Some(val) = from_node.get(current) {
        result.push(*val);
        current = val
    }
    result.reverse();
    result
}

// Implements A* algorithm for the problem
fn search_path(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let h = h(end);

    // TODO: Improve performance with binary heap. But how to update the values with its fscore?
    let mut open = HashSet::<(usize, usize)>::from([start]);
    let mut from_node = HashMap::<(usize, usize), (usize, usize)>::new();

    let mut gscore = HashMap::<(usize, usize), u32>::new();
    gscore.insert(start, 0);

    let mut fscore = HashMap::<(usize, usize), u32>::new();
    fscore.insert(start, h(start));

    while !open.is_empty() {
        let current = min(&open, &fscore);

        if current.0 == end.0 && current.1 == end.1 {
            return Some(build_path(&from_node, &current));
        }

        open.remove(&current);

        let current_score = *gscore.get(&current).unwrap();
        let neighbours = neighbours(map, current);

        for (neighbour, score) in neighbours {
            let tentative_score = current_score + score;
            
            if tentative_score < *gscore.get(&neighbour).unwrap_or(&u32::MAX) {
                from_node.insert(neighbour, current);
                gscore.insert(neighbour, tentative_score);
                fscore.insert(neighbour, tentative_score + h(neighbour));
                if !open.contains(&neighbour) {
                    open.insert(neighbour);
                }
            }
        }
    }
    None
}


#[aoc_generator(day20)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

fn cheats(input: &Input, start: (usize, usize), end: (usize, usize), target: usize) ->
    Vec<((usize, usize), usize)>
{
    let mut input = input.clone();
    let mut result = Vec::new();

    for y in 0 .. input.len() {
        for x in 0 .. input[y].len() {
            println!("{} / {}", y * input.len() + x, input.len() * input[y].len());
            
            let a = input[y][x];
            input[y][x] = 'C';

            let p = search_path(&input, start, end).unwrap();
            if p.len() - 1 < target {
                let t2 = p.len() - 1;
                result.push(((x, y), target - t2));
            }
            input[y][x] = a;
        }
    }
    result
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> usize {
    let start = find(&input, 'S');
    let end = find(&input, 'E');

    let path = search_path(&input, start, end).unwrap();

    let cheats = cheats(&input, start, end, path.len() - 1);

    cheats
        .iter()
        .filter(|(p,saving)| *saving >= 100)
        .count()
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Input {
        parse(
            "###############
             #...#...#.....#
             #.#.#.#.#.###.#
             #S#...#.#.#...#
             #######.#.#.###
             #######.#.#...#
             #######.#.###.#
             ###..E#...#...#
             ###.#######.###
             #...###...#...#
             #.#####.#.###.#
             #.#...#.#.#...#
             #.#.#.#.#.#.###
             #...#...#...###
             ###############"
        )
    }
    
    #[test]
    fn test_parse() {
        let input = sample_input();
        assert_eq!(input[0][0], '#');
        assert_eq!(input[1][0], '#');
        assert_eq!(input[1][1], '.');
        assert_eq!(input[3][1], 'S');
        assert_eq!(input[7][5], 'E');
    }

    #[test]
    fn test_search_path() {
        let input = sample_input();
        let start = find(&input, 'S');
        let end = find(&input, 'E');
        let path = search_path(&input, start, end).unwrap();

        print_map(&input, &path);
        
        assert_eq!(path.len() - 1, 84);
    }

    #[test]
    fn test_cheats() {
        let input = sample_input();
        let start = find(&input, 'S');
        let end = find(&input, 'E');

        let path = search_path(&input, start, end).unwrap();

        let cheats = cheats(&input, start, end, path.len() - 1);

        for c in &cheats {
            println!("{c:?}");
        }
        assert_eq!(cheats.len(), 44);
    }

}

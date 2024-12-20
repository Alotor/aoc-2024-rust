#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashMap,HashSet};
use colored::Colorize;

type Input = Vec<(usize, usize)>;

#[aoc_generator(day18)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let points: Vec<usize> = l.trim().split(",").map(|e| e.parse().unwrap()).collect();
            (points[0], points[1])
        })
        .collect()
}

fn gen_map(width: usize, height: usize, input: &Input, bytes: usize) -> Vec<Vec<char>> {
    let mut map = Vec::<Vec<char>>::with_capacity(height);
    for i in 0 .. height {
        let mut v = Vec::<char>::with_capacity(width);
        v.resize(width, '.');
        map.push(v);
    }

    for i in 0 .. bytes {
        let (x, y) = input[i];
        map[y][x] = '#';
    }
    map
}

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0 .. map.len() {
        for x in 0 .. map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}

fn print_path(map: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) {
    let points = HashSet::<&(usize, usize)>::from_iter(path);

    for y in 0 .. map.len() {
        for x in 0 .. map[y].len() {
            if points.contains(&(x, y)) {
                print!("{}", "O".red());
            } else {
                print!("{}", map[y][x]);
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
    if x > 0 && map[y][x-1] == '.' {
        result.push(((x - 1, y), 1));
    }
    if x < map[y].len() - 1 && map[y][x+1] == '.' {
        result.push(((x + 1, y), 1));
    }
    if y > 0 && map[y-1][x] == '.' {
        result.push(((x, y - 1), 1));
    }
    if y < map.len() - 1 && map[y+1][x] == '.' {
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
#[aoc(day18, part1)]
fn part1(input: &Input) -> usize {
    let map = gen_map(71, 71, &input, 1024);
    let path = search_path(&map, (0, 0), (70, 70)).unwrap();
    // print_path(&map, &path);
    path.len() - 1
}

fn block_path(width: usize, height: usize, start: (usize, usize), end: (usize, usize), input: &Input) -> (usize, usize) {

    fn block_path_rec(width: usize, height: usize, start: (usize, usize), end: (usize, usize), input: &Input,
                      from_i: usize, to_i: usize) -> usize
    {
        if from_i == to_i || from_i > to_i || from_i + 1 == to_i {
            from_i
        } else {
            let cur = from_i + (to_i - from_i) / 2;
            let map = gen_map(width, height, &input, cur);
            match search_path(&map, start, end) {
                None => {
                    block_path_rec(width, height, start, end, input, from_i, cur)
                },
                Some(_) => {
                    block_path_rec(width, height, start, end, input, cur, to_i)
                },
            }
        }
    }

    let i = block_path_rec(width, height, start, end, input, 0, input.len());
    input[i]
}

#[aoc(day18, part2)]
fn part2(input: &Input) -> String {
    let r = block_path(71, 71, (0,0), (70,70), &input);
    format!("{},{}", r.0, r.1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = parse(
            "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1
             1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0"
        );
        assert_eq!(input.len(), 25);
        assert_eq!(input[0].0, 5);
        assert_eq!(input[0].1, 4);
        assert_eq!(input[24].0, 2);
        assert_eq!(input[24].1, 0);
    }

    #[test]
    fn test_gen_map() {
        let input = parse(
            "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1
             1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0"
        );

        let map = gen_map(7, 7, &input, 12);
        print_map(&map);
        assert_eq!(map[0][0], '.');
        assert_eq!(map[0][3], '#');
        assert_eq!(map[1][0], '.');
        assert_eq!(map[1][3], '.');
        assert_eq!(map[1][2], '#');
    }

    #[test]
    fn test_min_path() {
        let input = parse(
            "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1
             1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0"
        );

        let map = gen_map(7, 7, &input, 12);
        let path = search_path(&map, (0, 0), (6, 6)).unwrap();
        print_path(&map, &path);
        assert_eq!(path.len() - 1, 22);
    }

    #[test]
    fn test_block_path() {
        let input = parse(
            "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1
             1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0"
        );

        let block = block_path(7, 7, (0,0), (6, 6), &input);
        assert_eq!(block, (6, 1));
    }
}

#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::{thread, time};
use std::collections::{HashSet, HashMap, BinaryHeap};
use colored::Colorize;

type Input = Vec<Vec<char>>;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Dir {
    N, S, E, W
}

use Dir::*;
    
#[aoc_generator(day16)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

fn start_position(input: &Input) -> (usize, usize) {
    for i in 0 .. input.len() {
        for j in 0 .. input.len() {
            if input[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("Map without start is not valid");
}

fn end_position(input: &Input) -> (usize, usize) {
    for i in 0 .. input.len() {
        for j in 0 .. input.len() {
            if input[i][j] == 'E' {
                return (i, j);
            }
        }
    }
    panic!("Map without end is not valid");
}

fn h(end: (usize, usize)) -> impl Fn((usize, usize, Dir)) -> u32 {
    move |(i, j, _)| {
        let i = i as f32;
        let j = j as f32;
        let a = end.0 as f32;
        let b = end.1 as f32;

        let res = ((i - a) * (i - a) + (j - b) * (j - b)).sqrt();

        res as u32    
    }
}

fn min(open: &HashSet::<(usize, usize, Dir)>, fscore: &HashMap::<(usize, usize, Dir), u32>) -> (usize, usize, Dir) {
    *open
        .iter()
        .min_by(|p1, p2| {
            let s1 = fscore.get(p1).unwrap_or(&u32::MAX);
            let s2 = fscore.get(p2).unwrap_or(&u32::MAX);
            s1.cmp(s2)
        })
        .unwrap()
}

fn neighbours(input: &Input, (i, j, dir): (usize, usize, Dir)) -> Vec<((usize, usize, Dir), u32)> {
    match dir {
        N => {
            let mut result = vec![
                ((i, j, E), 1000),
                ((i, j, W), 1000),
            ];

            if i > 0 && input[i-1][j] != '#' {
                result.push(((i - 1, j, N), 1))
            }
            result
        }

        S => {
            let mut result = vec![
                ((i, j, W), 1000),
                ((i, j, E), 1000),
            ];

            if i < input.len() - 1 && input[i+1][j] != '#' {
                result.push(((i + 1, j, S), 1))
            }
            result
        }
        E => {
            let mut result = vec![
                ((i, j, S), 1000),
                ((i, j, N), 1000),
            ];

            if j < input[i].len() - 1 && input[i][j+1] != '#' {
                result.push(((i, j+1, E), 1))
            }
            result
        }
        W => {
            let mut result = vec![
                ((i, j, S), 1000),
                ((i, j, N), 1000),
            ];

            if j > 0 && input[i][j-1] != '#' {
                result.push(((i, j-1, W), 1))
            }
            result
        }
    }
}

fn build_path(from_node: &HashMap::<(usize, usize, Dir), (usize, usize, Dir)>, last: &(usize, usize, Dir)) -> Vec<(usize, usize, Dir)> {
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
fn search_path(input: &Input, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<(usize, usize, Dir)>> {
    let h = h(end);
    let start = (start.0, start.1, E);

    // TODO: Improve performance with binary heap. But how to update the values with its fscore?
    let mut open = HashSet::<(usize, usize, Dir)>::from([start]);
    let mut from_node = HashMap::<(usize, usize, Dir), (usize, usize, Dir)>::new();

    let mut gscore = HashMap::<(usize, usize, Dir), u32>::new();
    gscore.insert(start, 0);

    let mut fscore = HashMap::<(usize, usize, Dir), u32>::new();
    fscore.insert(start, h(start));

    let mut min_path_score = u32::MAX;
    let mut all_paths = Vec::<Vec<(usize, usize, Dir)>>::new();

    while !open.is_empty() {
        let current = min(&open, &fscore);

        if current.0 == end.0 && current.1 == end.1 {
            let path = build_path(&from_node, &current);
            let score = score_path(&path);

            // println!("{score:?} - {path:?}");
            // all_paths.push(path);

            
            if score > min_path_score {
                // println!(">Exit");
                // This path is wors so we return
                return all_paths;
            } else {
                // Another path is added to the list
                min_path_score = score;
                all_paths.push(path);
            }
        }

        open.remove(&current);

        let current_score = *gscore.get(&current).unwrap();
        let neighbours = neighbours(input, current);

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
    // panic!("No path found")
    all_paths
}

fn print_path(input: &Vec<Vec<char>>, path: &Vec<(usize, usize, Dir)>) {
    let mut output = input.clone();

    for (i, j, dir) in path {
        output[*i][*j] = match dir {
            N => '^',
            S => 'v',
            E => '>',
            W => '<',
        }
    }

    for i in 0 .. output.len() {
        for j in 0 .. output[i].len() {
            if output[i][j] == '#' {
                print!("{}", "#".blue());
            } else if output[i][j] == '.' {
                print!(" ");
            } else {
                print!("{}", output[i][j].to_string().red());
            }
        }
        println!()
    }
}

fn print_all_path(input: &Vec<Vec<char>>, paths: &Vec<Vec<(usize, usize, Dir)>>) {
    let mut output = input.clone();

    let mut positions = HashSet::<(usize, usize)>::new();

    paths
        .iter()
        .flat_map(|p| p)
        .for_each(|(i, j, _)| {
            positions.insert((*i, *j));
        });
        
    for i in 0 .. output.len() {
        for j in 0 .. output[i].len() {
            if output[i][j] == '#' {
                print!("{}", "#".blue());
            } else if positions.contains(&(i, j)){
                print!("{}", "O".red());
            } else if output[i][j] == '.' {
                print!(" ");
            }
        }
        println!()
    }
}

fn score_path(path: &Vec<(usize, usize, Dir)>) -> u32 {
    let mut points = 0;
    for w in path.windows(2) {
        if w[0].2 == w[1].2 {
            // Same direction, +1
            points += 1
        } else {
            // Turn +1000
            points += 1000
        }
    }

    points   
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> u32 {
    let start = start_position(&input);
    let end = end_position(&input);
    let path = &search_path(&input, start, end)[0];

    // print_path(&input, &path);

    score_path(&path)
}

// #[aoc(day16, part2)]
fn part2(input: &Input) -> u32 {
    let start = start_position(&input);
    let end = end_position(&input);
    let paths = search_path(&input, start, end);

    print_all_path(&input, &paths);

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> Input {
        parse(
            "###############
             #.......#....E#
             #.#.###.#.###.#
             #.....#.#...#.#
             #.###.#####.#.#
             #.#.#.......#.#
             #.#.#####.###.#
             #...........#.#
             ###.#.#####.#.#
             #...#.....#.#.#
             #.#.#.###.#.#.#
             #.....#...#.#.#
             #.###.#.#.#.#.#
             #S..#.....#...#
             ###############"
        )
    }
    
    #[test]
    fn test_parse() {
        let input = default_input();
        assert_eq!(input.len(), 15);
        assert_eq!(input[0].len(), 15);
        assert_eq!(start_position(&input), (13, 1));
        assert_eq!(end_position(&input), (1, 13));
    }

    #[test]
    fn test_neighbours() {
        let input = default_input();
        assert_eq!(neighbours(&input, (13, 1, E)),
                   vec![((13, 1, S), 1000),
                        ((13, 1, N), 1000),
                        ((13, 2, E), 1)]);

        assert_eq!(neighbours(&input, (13, 1, W)),
                   vec![((13, 1, S), 1000),
                        ((13, 1, N), 1000)]);

        assert_eq!(neighbours(&input, (1, 1, N)),
                   vec![((1, 1, E), 1000),
                        ((1, 1, W), 1000)]);

        assert_eq!(neighbours(&input, (1, 1, S)),
                   vec![((1, 1, W), 1000),
                        ((1, 1, E), 1000),
                        ((2, 1, S), 1)]);
        
    }

    #[test]
    fn test_h(){
        let input = default_input();
        let end = end_position(&input);
        let h = h(end);

        // max distance from start
        assert_eq!(h((13, 1, N)), 16);
        assert_eq!(h((5, 1, N)), 12);
        assert_eq!(h((1, 1, N)), 12);
        assert_eq!(h((1, 5, N)), 8);

        // min distance at target
        assert_eq!(h((1, 13, N)), 0);
    }
    
    #[test]
    fn test_min(){
        let input = default_input();
        let mut open = HashSet::<(usize, usize, Dir)>::new();
        open.insert((0, 0, N));
        open.insert((1, 1, S));
        open.insert((3, 3, E));
        open.insert((4, 4, W));
        
        let mut fscore = HashMap::<(usize, usize, Dir), u32>::new();
        fscore.insert((0,0,N), 10);
        fscore.insert((1,1,S), 0);
        fscore.insert((3,3,E), 10);

        assert_eq!(min(&open, &fscore), (1,1,S));
    }

    #[test]
    fn test_search(){
        let input = default_input();
        let path = &search_path(&input, (13,1), (1,13))[0];
        print_path(&input, &path);
        assert_eq!(path.len(), 44);
    }

    #[test]
    fn test_search_2(){
        let input = parse(
            "#################
             #...#...#...#..E#
             #.#.#.#.#.#.#.#.#
             #.#.#.#...#...#.#
             #.#.#.#.###.#.#.#
             #...#.#.#.....#.#
             #.#.#.#.#.#####.#
             #.#...#.#.#.....#
             #.#.#####.#.###.#
             #.#.#.......#...#
             #.#.###.#####.###
             #.#.#...#.....#.#
             #.#.#.#####.###.#
             #.#.#.........#.#
             #.#.#.#########.#
             #S#.............#
             #################"
        );

        let start = start_position(&input);
        let end = end_position(&input);
        let path = &search_path(&input, start, end)[0];
        print_path(&input, &path);
        assert_eq!(path.len(), 60);
    }

    #[test]
    fn test_part1(){
        let input = default_input();
        assert_eq!(part1(&input), 7036);
    }

    #[test]
    fn test_part1_2(){
        let input = parse(
            "#################
             #...#...#...#..E#
             #.#.#.#.#.#.#.#.#
             #.#.#.#...#...#.#
             #.#.#.#.###.#.#.#
             #...#.#.#.....#.#
             #.#.#.#.#.#####.#
             #.#...#.#.#.....#
             #.#.#####.#.###.#
             #.#.#.......#...#
             #.#.###.#####.###
             #.#.#...#.....#.#
             #.#.#.#####.###.#
             #.#.#.........#.#
             #.#.#.#########.#
             #S#.............#
             #################"
        );

        assert_eq!(part1(&input), 11048);
    }

    // #[test]
    fn test_part2(){
        let input = default_input();
        assert_eq!(part2(&input), 45);
    }

    // #[test]
    fn test_part2_2(){
        let input = parse(
            "#################
             #...#...#...#..E#
             #.#.#.#.#.#.#.#.#
             #.#.#.#...#...#.#
             #.#.#.#.###.#.#.#
             #...#.#.#.....#.#
             #.#.#.#.#.#####.#
             #.#...#.#.#.....#
             #.#.#####.#.###.#
             #.#.#.......#...#
             #.#.###.#####.###
             #.#.#...#.....#.#
             #.#.#.#####.###.#
             #.#.#.........#.#
             #.#.#.#########.#
             #S#.............#
             #################"
        );

        assert_eq!(part2(&input), 64);
    }
}

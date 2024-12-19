#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashSet, HashMap};
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
        for j in 0 .. input[i].len() {
            if input[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("Map without start is not valid");
}

fn end_position(input: &Input) -> (usize, usize) {
    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            if input[i][j] == 'E' {
                return (i, j);
            }
        }
    }
    panic!("Map without end is not valid");
}

fn neighbours_prev(_input: &Input, (i, j, dir): (usize, usize, Dir)) -> Vec<((usize, usize, Dir), u32)> {
    let mut result = Vec::new();
    match dir {
        N => {
            result.push(((i+1, j, N), 1));
            result.push(((i, j, E), 1000));
            result.push(((i, j, W), 1000));
        }
        S => {
            result.push(((i-1, j, S), 1));
            result.push(((i, j, E), 1000));
            result.push(((i, j, W), 1000));
        }
        E => {
            result.push(((i, j-1, E), 1));
            result.push(((i, j, N), 1000));
            result.push(((i, j, S), 1000));
        }
        W => {
            result.push(((i, j+1, W), 1));
            result.push(((i, j, N), 1000));
            result.push(((i, j, S), 1000));
        }
    };
    result
}

fn neighbours(input: &Input, (i, j, dir): (usize, usize, Dir)) -> Vec<((usize, usize, Dir), u32)> {
    let mut result = Vec::new();
    match dir {
        N => {
            if i > 0 && input[i-1][j] != '#' {
                result.push(((i - 1, j, N), 1))
            }
            result.push(((i, j, E), 1000));
            result.push(((i, j, W), 1000));
        }

        S => {
            if i < input.len() - 1 && input[i+1][j] != '#' {
                result.push(((i + 1, j, S), 1))
            }
            result.push(((i, j, W), 1000));
            result.push(((i, j, E), 1000));
        }
        E => {
            if j < input[i].len() - 1 && input[i][j+1] != '#' {
                result.push(((i, j+1, E), 1))
            }
            result.push(((i, j, S), 1000));
            result.push(((i, j, N), 1000));
        }
        W => {
            if j > 0 && input[i][j-1] != '#' {
                result.push(((i, j-1, W), 1))
            }
            result.push(((i, j, S), 1000));
            result.push(((i, j, N), 1000));
        }
    };
    result
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

fn search_path(input: &Input, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize, Dir)> {
    let mut all_points = HashSet::<(usize, usize)>::new();
    search_path_aux(input, start, end, &mut all_points)
}

// Implements A* algorithm for the problem
fn search_path_aux(input: &Input, start: (usize, usize), end: (usize, usize), all_nodes: &mut HashSet<(usize, usize)>) -> Vec<(usize, usize, Dir)> {
    let start = (start.0, start.1, E);

    let mut open = HashSet::<(usize, usize, Dir)>::from([start]);
    let mut from_node = HashMap::<(usize, usize, Dir), (usize, usize, Dir)>::new();

    let mut gscore = HashMap::<(usize, usize, Dir), u32>::new();
    gscore.insert(start, 0);

    while !open.is_empty() {
        let current = *open.iter().next().unwrap();
        open.remove(&current);

        let current_score = *gscore.get(&current).unwrap();
        let neighbours = neighbours(input, current);

        for (neighbour, score) in neighbours {
            let tentative_score = current_score + score;
            
            if tentative_score < *gscore.get(&neighbour).unwrap_or(&u32::MAX) {
                from_node.insert(neighbour, current);
                gscore.insert(neighbour, tentative_score);
                if !open.contains(&neighbour) {
                    open.insert(neighbour);
                }
            }
        }
    }

    all_nodes.insert(end);
    all_nodes.insert((start.0, start.1));

    let end_nodes: Vec<_> = [N, S, W, E]
        .iter()
        .map(|d| (end.0, end.1, *d))
        .collect();
    
    let min_end_score = end_nodes
        .iter()
        .map(|n| gscore.get(&n).unwrap_or(&10000000))
        .min().unwrap();


    let min_end: Vec<(usize,usize,Dir)> = end_nodes
        .iter()
        .filter(|n| gscore.get(&n).unwrap_or(&10000000) == min_end_score)
        .map(|n| (n.0, n.1, n.2))
        .collect();

    let mut pending = Vec::from(min_end);

    while let Some(current) = pending.pop() {
        let nns = neighbours_prev(input, current);
        
        let min = nns
            .iter()
            .map(|(n, s)| gscore.get(n).unwrap_or(&10000000) + s)
            .min().unwrap();

        let path_nodes: Vec<_> = nns
            .iter()
            .filter(|(n,s)| gscore.get(n).unwrap_or(&10000000) + s == min)
            .collect();
        
        path_nodes.iter().for_each(|(n, _)| {
            if *n != start {
                all_nodes.insert((n.0, n.1));
                pending.push(*n);
            } 
        });
    }

    let end_node = end_nodes.iter().min_by(|a,b| {
        let s1 = gscore.get(a).unwrap_or(&10000000);
        let s2 = gscore.get(b).unwrap_or(&10000000);
        s1.cmp(s2)
    }).unwrap();
    
    let path = build_path(&from_node, &end_node);
    path
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

fn print_nodes(input: &Vec<Vec<char>>, nodes: &HashSet<(usize, usize)>) {
    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            if input[i][j] == '#' {
                print!("{}", "#".blue());
            } else if nodes.contains(&(i, j)){
                print!("{}", "O".red());
            } else if input[i][j] == '.' {
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
    let path = search_path(&input, start, end);

    print_path(&input, &path);

    score_path(&path)
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> u32 {
    let start = start_position(&input);
    let end = end_position(&input);

    let mut visited = HashSet::<(usize, usize)>::new();
    search_path_aux(&input, start, end, &mut visited);

    print_nodes(&input, &visited);
    visited.len() as u32
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
                   vec![((13, 2, E), 1),
                        ((13, 1, S), 1000),
                        ((13, 1, N), 1000)]);

        assert_eq!(neighbours(&input, (13, 1, W)),
                   vec![((13, 1, S), 1000),
                        ((13, 1, N), 1000)]);

        assert_eq!(neighbours(&input, (1, 1, N)),
                   vec![((1, 1, E), 1000),
                        ((1, 1, W), 1000)]);

        assert_eq!(neighbours(&input, (1, 1, S)),
                   vec![((2, 1, S), 1),                        
                        ((1, 1, W), 1000),
                        ((1, 1, E), 1000)]);
    }

    #[test]
    fn test_search(){
        let input = default_input();
        let path = search_path(&input, (13,1), (1,13));
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
        let path = search_path(&input, start, end);
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

    #[test]
    fn test_part2(){
        let input = default_input();
        assert_eq!(part2(&input), 45);
    }

    #[test]
    fn test_part2_3(){
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

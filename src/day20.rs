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

fn print_map(map: &Vec<Vec<char>>, path: &Vec<(usize, usize)>, cheat: (usize, usize)) {
    let path_points = HashSet::<&(usize, usize)>::from_iter(path);
    for i in 0 .. map.len() {
        for j in 0 .. map[i].len() {
            if map[i][j] == 'S' || map[i][j] == 'E' {
                print!("{}", map[i][j].to_string().yellow());
            } else if j == cheat.0 && i == cheat.1 {
                print!("{}", "X".green());
            } else if path_points.contains(&(j, i)) {
                if map[i][j] == '#' {
                    print!("{}", "X".red());
                } else {
                    print!("{}", "O".red());
                }
                
            } else {
                print!("{}", map[i][j].to_string().blue());
            }
        }
        println!();
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Node {
    x: usize,
    y: usize,
    cheat: Option<(usize, usize)>,
    cheat_steps: usize,
}

impl Node {
    fn new(prev: Node, x: usize, y: usize, end: (usize, usize)) -> Self {
        let cheat = prev.cheat;
        let mut cheat_steps = prev.cheat_steps;
        if cheat != None && cheat_steps > 0 {
            cheat_steps -= 1;
        }
        Self { x, y, cheat, cheat_steps: if (x,y) == end {0} else {cheat_steps} }
    }
    fn from_pos(pos: (usize, usize), cheat_steps: usize) -> Self {
        Self { x: pos.0, y: pos.1, cheat: None, cheat_steps }
    }
    fn cheat(prev: Node, x: usize, y: usize, end: (usize, usize)) -> Self {
        let cheat = if prev.cheat == None { Some((x, y)) } else { prev.cheat };
        let cheat_steps = prev.cheat_steps - 1;
        Self { x, y, cheat, cheat_steps: if (x,y) == end {0} else {cheat_steps} }
    }
}

fn neighbours(map: &Vec<Vec<char>>, node: Node, cheat_length: usize, path_nodes: &HashSet<(usize, usize)>, end: (usize, usize)) -> Vec<(Node, u32)> {
    let mut result = Vec::new();

    let x = node.x;
    let y = node.y;

    if map[y][x] == '#' && node.cheat_steps == 0 {
        return result;
    }
    
    if x > 0 && map[y][x-1] != '#' && !path_nodes.contains(&(x-1, y)) {
        result.push((Node::new(node, x - 1, y, end), 1));
    }
    if x < map[y].len() - 1 && map[y][x+1] != '#' && !path_nodes.contains(&(x+1, y)) {
        result.push((Node::new(node, x + 1, y, end), 1));
    }
    if y > 0 && map[y-1][x] != '#' && !path_nodes.contains(&(x, y-1)) {
        result.push((Node::new(node, x, y - 1, end), 1));
    }
    if y < map.len() - 1 && map[y+1][x] != '#' && !path_nodes.contains(&(x, y+1)) {
        result.push((Node::new(node, x, y + 1, end), 1));
    }

    if x > 0 && map[y][x-1] == '#' && node.cheat_steps > 0 && !path_nodes.contains(&(x-1, y)) {
        result.push((Node::cheat(node, x - 1, y, end), 1));
    }
    if x < map[y].len() - 1 && map[y][x+1] == '#' && node.cheat_steps > 0 && !path_nodes.contains(&(x+1, y)) {
        result.push((Node::cheat(node, x + 1, y, end), 1));
    }
    if y > 0 && map[y-1][x] == '#' && node.cheat_steps > 0 && !path_nodes.contains(&(x, y-1)) {
        result.push((Node::cheat(node, x, y - 1, end), 1));
    }
    if y < map.len() - 1 && map[y+1][x] == '#' && node.cheat_steps > 0 && !path_nodes.contains(&(x, y+1)) {
        result.push((Node::cheat(node, x, y + 1, end), 1));
    }

    result
}

fn build_path(from_node: &HashMap::<Node, Node>, last: &Node) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::from([(last.x, last.y)]);
    let mut current = last;

    while let Some(val) = from_node.get(current) {
        result.push((val.x, val.y));
        current = val
    }
    result.reverse();
    result
}

fn h(end: (usize, usize)) -> impl Fn(Node) -> u32 {
    move |node| {
        let nx = node.x as f32;
        let ny = node.y as f32;
        let ex = end.0 as f32;
        let ey = end.1 as f32;

        let res = ((nx - ey) * (nx - ey) + (ny - ey) * (ny - ey)).sqrt();
        res as u32
    }
}

fn min(open: &HashSet::<Node>, fscore: &HashMap::<Node, u32>) -> Node {
    *open
        .iter()
        .min_by(|p1, p2| {
            let s1 = *fscore.get(p1).unwrap_or(&u32::MAX);
            let s2 = *fscore.get(p2).unwrap_or(&u32::MAX);
            s1.cmp(&s2)
        })
        .unwrap()
}

fn current_path_nodes(from_node: &HashMap::<Node, Node>, current: &Node) -> HashSet<(usize, usize)> {
    let mut result = HashSet::<(usize, usize)>::from([(current.x, current.y)]);
    let mut current = current;

    while let Some(val) = from_node.get(current) {
        result.insert((val.x, val.y));
        current = val
    }
    result
}

// Implements A* algorithm for the problem
fn search_cheats(
    input: &Input, start: (usize, usize), end: (usize, usize), cheat_length: usize) -> Vec<u32> {

    let h = h(end);

    let start = Node::from_pos(start, cheat_length);

    let mut open = HashSet::<Node>::from([start]);
    let mut from_node = HashMap::<Node, Node>::new();

    let mut gscore = HashMap::<Node, u32>::new();
    gscore.insert(start, 0);

    let mut fscore = HashMap::<Node, u32>::new();
    fscore.insert(start, h(start));

    let mut end_nodes = HashSet::<Node>::new();

    let mut i = 0;
    while !open.is_empty() {
        let current = min(&open, &fscore);

        println!("{current:?}");

        open.remove(&current);

        let current_score = *gscore.get(&current).unwrap();
        let path_nodes = current_path_nodes(&from_node, &current);
        let neighbours = neighbours(input, current, cheat_length, &path_nodes, end);

        for (neighbour, score) in neighbours {
            println!("+++ {neighbour:?}");
            
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
        if current.x == end.0 && current.y == end.1 {
            end_nodes.insert(current);
        }
        i += 1;

        if i > 50 {
            break;
        }
    }



    /*
    
    let start = Node::from_pos(start, cheat_length);

    let mut open = HashSet::<Node>::from([start]);
    let mut from_node = HashMap::<Node, Node>::new();

    let mut gscore = HashMap::<Node, u32>::new();
    gscore.insert(start, 0);

    let mut end_nodes = HashSet::<Node>::new();

    let mut i = 0;
    while !open.is_empty() {
        if i % 1000000 == 0 {
            // println!("{}: {}", i / 1000000, open.len());
        }
        let current = min(&open, &gscore);
        open.remove(&current);

        let current_score = *gscore.get(&current).unwrap();
        let path_nodes = current_path_nodes(&from_node, &current);
        let neighbours = neighbours(input, current, cheat_length, &path_nodes);

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

        if current.x == end.0 && current.y == end.1 {
            end_nodes.insert(current);
        }

        i += 1;
}
    */

    let mut result = Vec::new();

    let base_node = Node {x: end.0, y: end.1, cheat: None, cheat_steps: 0};
    let base = gscore.get(&base_node).unwrap_or(&100000);

    let mut min_node = base_node;
    let mut min_score = base;

    for node in end_nodes {
        let score = gscore.get(&node).unwrap_or(&0);
        if score < base {
            result.push(base - score);

            // if node.cheat == Some((6,7)) {
            //     println!("{node:?}");
            //     print_map(&input, &build_path(&from_node, &node), node.cheat.unwrap());
            // }
            
            if base - score > 50 {
                println!("{} {node:?}", base - score);
                print_map(&input, &build_path(&from_node, &node), node.cheat.unwrap());
            }
        }
    }
    result
}


#[aoc_generator(day20)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

/*
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
*/

#[aoc(day20, part1)]
fn part1(input: &Input) -> usize {
    let start = find(&input, 'S');
    let end = find(&input, 'E');

    let cheats = search_cheats(&input, start, end, 2);

    cheats
        .iter()
        .filter(|saving| **saving >= 100)
        .count()
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> usize {
    let start = find(&input, 'S');
    let end = find(&input, 'E');

    let cheats = search_cheats(&input, start, end, 6);

    cheats
        .iter()
        .filter(|saving| **saving >= 100)
        .count()
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

    /*
    #[test]
    fn test_search_path() {
        let input = sample_input();
        let start = find(&input, 'S');
        let end = find(&input, 'E');

        let path = search_path(&input, start, end, 2);

        // print_map(&input, &path);
        
        // assert_eq!(path.len() - 1, 84);
}
    */

    #[test]
    fn test_cheats() {
        let input = sample_input();
        let start = find(&input, 'S');
        let end = find(&input, 'E');

        let cheats = search_cheats(&input, start, end, 2);
        assert_eq!(cheats.len(), 44);
    }

    #[test]
    fn test_cheats_2() {
        let input = sample_input();
        let start = find(&input, 'S');
        let end = find(&input, 'E');

        let cheats = search_cheats(&input, start, end, 6);
        let cc: Vec<_> = cheats.iter().filter(|saving| **saving > 50).collect();

        println!("{cc:?}");
        assert_eq!(cheats.iter().filter(|saving| **saving == 50).count(), 32);
        //assert_eq!(cheats.iter().filter(|saving| **saving == 52).count(), 32);
        //assert_eq!(cheats.iter().filter(|saving| **saving == 54).count(), 29);
        //assert_eq!(cheats.iter().filter(|saving| **saving == 56).count(), 39);
    }
}

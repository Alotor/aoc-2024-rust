#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashMap, HashSet};
use colored::Colorize;

type Input = Vec<Vec<char>>;

const INF: usize = 10000000;

#[aoc_generator(day20)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

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

fn nbs(map: &Input, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if x > 0 && map[y][x-1] != '#' {
        result.push((x - 1, y));
    }
    if x < map[y].len() - 1 && map[y][x+1] != '#' {
        result.push((x + 1, y));
    }
    if y > 0 && map[y-1][x] != '#' {
        result.push((x, y - 1));
    }
    if y < map.len() - 1 && map[y+1][x] != '#' {
        result.push((x, y + 1));
    }

    result
}

fn build_distances(map: &Input, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<usize>> {

    let mut result = Vec::<Vec<usize>>::with_capacity(map.len());

    for y in 0 .. map.len() {
        result.push(Vec::with_capacity(map[y].len()));
        for x in 0 .. map[y].len() {
            result[y].push(INF);
        }
    }

    let mut pending = Vec::from([end]);
    result[end.1][end.0] = 0;
    
    while let Some(p @ (x, y)) = pending.pop() {
        let cur_d = result[y][x];
        let nbss = nbs(map, p);

        // println!("{p:?} {cur_d} {nbss:?}");
        for n in nbss {
            let (nx, ny) = n;
            if cur_d + 1 < result[ny][nx] {
                // println!(" +++ {n:?} {}", cur_d + 1);
                result[ny][nx] = cur_d + 1;
                pending.push(n);
            }
        }
    }

    result
}

fn get_nodes(map: &Input, (x, y): (usize, usize), radius: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();

    let radius = radius as i32;
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    for ix in -radius ..= radius {
        for iy in -radius ..= radius {
            if ix == 0 && iy == 0 {
                continue;
            }

            let x2 = x as i32 + ix;
            let y2 = y as i32 + iy;
            
            if x2 >= 0 && x2 < width && y2 >= 0 && y2 < height {
                let x2 = x2 as usize;
                let y2 = y2 as usize;
                let d = dist((x, y), (x2, y2));

                if d <= radius as usize {
                    result.push((x2 as usize, y2 as usize));
                }
            }
        }
    }

    result
}

fn dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    let x1 = x1 as i32;
    let y1 = y1 as i32;
    let x2 = x2 as i32;
    let y2 = y2 as i32;
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}

fn find_cheats(map: &Input, dm: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize), radius: usize, min_cheat: usize) -> Vec<usize> {
    let mut result = Vec::<usize>::new();
    
    for y in 0 .. map.len() {
        for x in 0 .. map[y].len() {
            if map[y][x] == '#' {
                continue;
            }
            
            let cur_dist = dm[y][x];

            // Get the minimun distance in the radius
            let nodes = get_nodes(map, (x, y), radius);
          
            let cheat_nodes = nodes
                .iter()
                .filter(|(nx, ny)| {
                    cur_dist >= dm[*ny][*nx] + dist((x,y), (*nx,*ny))
                })
                .for_each(|(ox, oy)| {
                    let new_dist = dm[*oy][*ox] + dist((x, y), (*ox, *oy));
                    let cheat_save = cur_dist - new_dist;

                    if new_dist < cur_dist && cheat_save >= min_cheat {
                        result.push(cheat_save);
                    }
                });
        }
    }
    result
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> usize {
    let start = find(&input, 'S');
    let end = find(&input, 'E');

    let dist = build_distances(&input, start, end);
    let cheats = find_cheats(&input, &dist, start, end, 2, 100);

    cheats.iter().count()
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> usize {
    let start = find(&input, 'S');
    let end = find(&input, 'E');

    let dist = build_distances(&input, start, end);
    let cheats = find_cheats(&input, &dist, start, end, 20, 100);

    cheats.iter().count()
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
    fn test_build_distances() {
        let map = sample_input();
        let start = find(&map, 'S');
        let end = find(&map, 'E');
        let dist = build_distances(&map, start, end);

        for y in 0 .. map.len() {
            for x in 0 .. map[y].len() {
                if map[y][x] == '#' {
                    print!("(###)");
                } else {
                    print!("({:03})", dist[y][x]);
                }
            }
            println!();
        }

        assert_eq!(dist[start.1][start.0], 84);
    }

    #[test]
    fn test_get_nodes() {
        let map = sample_input();

        assert_eq!(get_nodes(&map, (3, 3), 2).len(), 12);

        let n33_6 = HashSet::<(usize, usize)>::from_iter(get_nodes(&map, (3, 3), 6));
        assert_eq!(n33_6.len(), 66);
        assert!(n33_6.contains(&(3,9)));
        assert!(n33_6.contains(&(9,3)));
        assert!(n33_6.contains(&(3,0)));
        assert!(n33_6.contains(&(0,3)));
        assert!(n33_6.contains(&(6,0)));
        assert!(n33_6.contains(&(0,0)));
        assert!(n33_6.contains(&(0,6)));
        
    }

    #[test]
    fn test_cheats() {
        let map = sample_input();
        let start = find(&map, 'S');
        let end = find(&map, 'E');
        let dist = build_distances(&map, start, end);

        let cheats = find_cheats(&map, &dist, start, end, 2, 0);

        let mut m: HashMap<usize, usize> = HashMap::new();
        for c in &cheats {
            *m.entry(*c).or_default() += 1;
        }
        println!("{m:?}");
        
        assert_eq!(cheats.len(), 44);
    }


    #[test]
    fn test_cheats_6() {
        let map = sample_input();
        let start = find(&map, 'S');
        let end = find(&map, 'E');
        let dist = build_distances(&map, start, end);

        let cheats = find_cheats(&map, &dist, start, end, 20, 50);

        let mut m: HashMap<usize, usize> = HashMap::new();
        for c in &cheats {
            *m.entry(*c).or_default() += 1;
        }
        println!("{m:?}");

        assert_eq!(*m.get(&50).unwrap_or(&0), 32);
        assert_eq!(*m.get(&52).unwrap_or(&0), 31);
        assert_eq!(*m.get(&54).unwrap_or(&0), 29);
        assert_eq!(*m.get(&58).unwrap_or(&0), 25);
        assert_eq!(*m.get(&60).unwrap_or(&0), 23);
        assert_eq!(*m.get(&62).unwrap_or(&0), 20);
        assert_eq!(*m.get(&64).unwrap_or(&0), 19);
        assert_eq!(*m.get(&66).unwrap_or(&0), 12);
        assert_eq!(*m.get(&68).unwrap_or(&0), 14);
        assert_eq!(*m.get(&70).unwrap_or(&0), 12);
        assert_eq!(*m.get(&74).unwrap_or(&0), 4);
        assert_eq!(*m.get(&76).unwrap_or(&0), 3);
    }
    
}


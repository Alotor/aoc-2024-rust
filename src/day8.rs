#![allow(unused_variables, unused_mut, dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;

type Input = (i32, i32, HashMap<char, Vec<(i32, i32)>>);

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let mut entries = HashMap::new();

    let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    for i in 0 .. lines.len() {
        let line = lines[i];
        let chars: Vec<char> = line.chars().collect();

        for j in 0 .. line.len() {
            let c = chars[j];

            if c == '.' {
                continue;
            }
            if entries.contains_key(&c) {
                let mut cval: &mut Vec<(i32, i32)> = entries.get_mut(&c).unwrap();
                cval.push((i as i32, j as i32));
            } else {
                let mut cval = vec![(i as i32, j as i32)];
                entries.insert(c, cval);
            }
        }
    }

    (width, height, entries)
}

fn valid(width: &i32, height: &i32, (a, b): &(i32, i32)) -> bool {
    *a >= 0 && *a < *width && *b >= 0 && *b < *height
}

fn gen_antinodes(width: &i32, height: &i32, nodes: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for i in 0 .. nodes.len() {
        for j in 0 .. nodes.len() {
            if i == j {
                continue;
            }

            let (a1, b1) = nodes[i];
            let (a2, b2) = nodes[j];

            let a = a2 - a1;
            let b = b2 - b1;

            let anode = (a2 + a, b2 + b);

            if valid(width, height, &anode) {
                result.push(anode);
            }
        }
    }
    result
}

fn gen_antinodes_v2(width: &i32, height: &i32, nodes: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for i in 0 .. nodes.len() {
        for j in 0 .. nodes.len() {
            if i == j {
                continue;
            }

            let (a1, b1) = nodes[i];
            let (a2, b2) = nodes[j];

            let a = a2 - a1;
            let b = b2 - b1;

            let mut anode = (a1 + a, b1 + b);

            while valid(width, height, &anode) {
                result.push(anode);
                anode.0 += a;
                anode.1 += b;
            }
        }
    }
    result
}

#[aoc(day8, part1)]
fn part1((width, height, entries): &Input) -> u32 {
    let mut result = HashSet::new();

    for (c, nodes) in entries {
        let antinodes = gen_antinodes(width, height, nodes);
        for an in antinodes {
            result.insert(an);
        }
    }
    
    result.len() as u32
}

#[aoc(day8, part2)]
fn part2((width, height, entries): &Input) -> u32 {
    let mut result = HashSet::new();

    for (c, nodes) in entries {
        let antinodes = gen_antinodes_v2(width, height, nodes);
        for an in antinodes {
            result.insert(an);
        }
    }
    
    result.len() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "............
                     ........0...
                     .....0......
                     .......0....
                     ....0.......
                     ......A.....
                     ............
                     ............
                     ........A...
                     .........A..
                     ............
                     ............";

        let r = parse(input);
        assert_eq!(r.0, 12);
        assert_eq!(r.1, 12);
        assert_eq!(r.2.get(&'0').unwrap().len(), 4);
        assert_eq!(r.2.get(&'0').unwrap().get(0).unwrap(), &(1, 8));
        assert_eq!(r.2.get(&'A').unwrap().len(), 3);
    }

    #[test]
    fn test_gen_aninodes() {
        assert_eq!(gen_antinodes(&10, &10, &vec![(4, 3), (5, 5)]), vec![(6, 7), (3, 1)]);
        assert_eq!(gen_antinodes(&10, &10, &vec![(4, 3), (8, 4), (5, 5)]), vec![(6, 7), (0, 2), (2, 6), (3, 1)]);
    }

    #[test]
    fn test_gen_aninodes_v2() {
        assert_eq!(gen_antinodes_v2(&10, &9, &vec![(0, 0), (3, 1), (1, 2)]), vec![(3, 1), (6, 2), (9, 3), (1, 2), (2, 4), (3, 6), (4, 8), (0, 0), (1, 2), (0, 0), (3, 1), (5, 0)]);
    }

    #[test]
    fn test_part1() {
        let input = "............
                     ........0...
                     .....0......
                     .......0....
                     ....0.......
                     ......A.....
                     ............
                     ............
                     ........A...
                     .........A..
                     ............
                     ............";

        assert_eq!(part1(&parse(input)), 14);
    }

    #[test]
    fn test_part2() {
        let input = "............
                     ........0...
                     .....0......
                     .......0....
                     ....0.......
                     ......A.....
                     ............
                     ............
                     ........A...
                     .........A..
                     ............
                     ............";

        assert_eq!(part2(&parse(input)), 34);
    }
}

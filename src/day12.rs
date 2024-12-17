#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

type Input = Vec<Vec<char>>;

use crate::utils::get2d;
use std::collections::HashSet;
    
#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Block {
    id: char,
    elements: HashSet<(usize, usize)>,
}

fn neighbours(input: &Input, id: char, (i, j): (usize, usize)) -> Vec<(usize, usize)>{
    let i = i as i32;
    let j = j as i32;

    [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
        .iter()
        .filter(|(i, j)| *get2d(input, *i, *j) == id)
        .map(|(i,j)| (*i as usize, *j as usize))
        .collect()
}

fn num_neighbours(input: &Input, id: char, (i, j): (usize, usize)) -> u32 {
    let i = i as i32;
    let j = j as i32;

    [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
        .iter()
        .filter(|(i, j)| *get2d(input, *i, *j) == id)
        .map(|(i,j)| (*i as usize, *j as usize))
        .count() as u32
}

fn perimeter(input: &Input, block: &Block) -> u32 {
    let mut result = 0;

    for e in &block.elements {
        result += 4 - num_neighbours(input, block.id, *e);
    }
    
    result
}

fn resolve_block(input: &Input, i: usize, j: usize) -> Block {
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut pending = Vec::<(usize, usize)>::new();
    let mut elements = HashSet::<(usize, usize)>::new();
    let id = input[i][j];

    pending.push((i, j));

    while let Some(p) = pending.pop() {
        visited.insert(p);
        if input[p.0][p.1] == id {
            elements.insert(p);
            neighbours(input, id, p)
                .iter()
                .filter(|n| !visited.contains(n))
                .for_each(|n| {
                    pending.push(*n);
                });
        }
    }

    Block { id, elements }
}

fn find_blocks(input: &Input) -> Vec<Block>{
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut result = Vec::new();

    for i in 0 .. input.len() {
        for j in 0 .. input[i].len() {
            if !visited.contains(&(i, j)) {

                let block = resolve_block(input, i, j);
                block.elements.iter().for_each(|(i, j)| {
                    visited.insert((*i, *j));
                });
                result.push(block);
            }
            
        }
    }
    result
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Dir {
    Left, Right, Top, Bottom
}

use Dir::*;

fn num_sides(input: &Input, block: &Block) -> u32 {

    let mut unique_sides = HashSet::<(Dir, i32, i32)>::new();
    let mut all_sides = HashSet::<(Dir, i32, i32)>::new();

    let id = block.id;

    // Need to be sorted for it to work
    let mut elements = Vec::from_iter(&block.elements);
    elements.sort();

    for (i, j) in elements {
        let i = *i as i32;
        let j = *j as i32;

        // LEFT SIDE
        if *get2d(input, i, j - 1) != id {
            all_sides.insert((Left, i, j));

            if !all_sides.contains(&(Left, i + 1, j)) && !all_sides.contains(&(Left, i - 1, j)) {
                unique_sides.insert((Left, i, j));
            }
        }
        // RIGHT SIDE
        if *get2d(input, i, j + 1) != id {
            all_sides.insert((Right, i, j));

            if !all_sides.contains(&(Right, i + 1, j)) && !all_sides.contains(&(Right, i - 1, j)) {
                unique_sides.insert((Right, i, j));
            }
        }
        // TOP SIDE
        if *get2d(input, i - 1, j) != id {
            all_sides.insert((Top, i, j));

            if !all_sides.contains(&(Top, i, j + 1)) && !all_sides.contains(&(Top, i, j - 1)) {
                unique_sides.insert((Top, i, j));
            }
        }
        // BOTTOM SIDE
        if *get2d(input, i + 1, j) != id {
            all_sides.insert((Bottom, i, j));

            if !all_sides.contains(&(Bottom, i, j + 1)) && !all_sides.contains(&(Bottom, i, j -1)) {
                unique_sides.insert((Bottom, i, j));
            }
        }
        
    }
    
    unique_sides.len() as u32
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> u32 {

    find_blocks(input)
        .iter()
        .map(|block| {
            block.elements.len() as u32 * perimeter(input, &block)
        })
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> u32 {

    find_blocks(input)
        .iter()
        .map(|block| {
            block.elements.len() as u32 * num_sides(input, &block)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input =
            parse("AAAA
                   BBCD
                   BBCC
                   EEEC");
        assert_eq!(
            input,
            vec![
                vec!['A', 'A', 'A', 'A'],
                vec!['B', 'B', 'C', 'D'],
                vec!['B', 'B', 'C', 'C'],
                vec!['E', 'E', 'E', 'C'],
            ]
        );
    }

    #[test]
    fn test_resolve_block() {
        let input =
            parse("AAAAXXXX
                   BBCD---X
                   BBCCXXXX
                   EEEC-X-X");

        assert_eq!(
            resolve_block(&input, 0, 0),
            Block {
                id: 'A',
                elements: HashSet::from_iter([(0,0),(0,1),(0,2),(0,3)])
            }
        );

        assert_eq!(
            resolve_block(&input, 1, 0),
            Block {
                id: 'B',
                elements: HashSet::from_iter([(1,0),(1,1),(2,0),(2,1)])
            }
        );

        assert_eq!(
            resolve_block(&input, 0, 4),
            Block {
                id: 'X',
                elements: HashSet::from_iter([(0,4),(0,5),(0,6),(0,7),(1,7),(2,4),(2,5),(2,6),(2,7),(3,7),(3,5)])
            }
        );
    }

    #[test]
    fn test_find_blocks() {
        let input =
            parse("AAAA
                   BBCD
                   BBCC
                   EEEC");

        let blocks = find_blocks(&input);
        assert_eq!(blocks.len(), 5);

        assert_eq!(blocks[0].id, 'A');
        assert_eq!(blocks[0].elements.len(), 4);
        
        assert_eq!(blocks[1].id, 'B');
        assert_eq!(blocks[1].elements.len(), 4);

        assert_eq!(blocks[2].id, 'C');
        assert_eq!(blocks[2].elements.len(), 4);

        assert_eq!(blocks[3].id, 'D');
        assert_eq!(blocks[3].elements.len(), 1);

        assert_eq!(blocks[4].id, 'E');
        assert_eq!(blocks[4].elements.len(), 3);
    }

    #[test]
    fn test_find_blocks_edge() {
        let input =
            parse("OOOOO
                   OXOXO
                   OOOOO
                   OXOXO
                   OOOOO");

        let blocks = find_blocks(&input);
        assert_eq!(blocks.len(), 5);

        assert_eq!(blocks[0].id, 'O');
        assert_eq!(blocks[0].elements.len(), 21);
        
        assert_eq!(blocks[1].id, 'X');
        assert_eq!(blocks[1].elements.len(), 1);

        assert_eq!(blocks[2].id, 'X');
        assert_eq!(blocks[2].elements.len(), 1);

        assert_eq!(blocks[3].id, 'X');
        assert_eq!(blocks[3].elements.len(), 1);

        assert_eq!(blocks[4].id, 'X');
        assert_eq!(blocks[4].elements.len(), 1);
    }

    #[test]
    fn test_perimeter() {
        let input =
            parse("AAAA
                   BBCD
                   BBCC
                   EEEC");

        let blocks = find_blocks(&input);
        assert_eq!(blocks.len(), 5);

        assert_eq!(blocks[0].id, 'A');
        assert_eq!(perimeter(&input, &blocks[0]), 10);

        assert_eq!(blocks[1].id, 'B');
        assert_eq!(perimeter(&input, &blocks[1]), 8);

        assert_eq!(blocks[2].id, 'C');
        assert_eq!(perimeter(&input, &blocks[2]), 10);

        assert_eq!(blocks[3].id, 'D');
        assert_eq!(perimeter(&input, &blocks[3]), 4);

        assert_eq!(blocks[4].id, 'E');
        assert_eq!(perimeter(&input, &blocks[4]), 8);
    }

    #[test]
    fn test_part1() {
        let input =
            parse("RRRRIICCFF
                   RRRRIICCCF
                   VVRRRCCFFF
                   VVRCCCJFFF
                   VVVVCJJCFE
                   VVIVCCJJEE
                   VVIIICJJEE
                   MIIIIIJJEE
                   MIIISIJEEE
                   MMMISSJEEE");
        assert_eq!(part1(&input), 1930);
    }

    #[test]
    fn test_numsides_basic() {
        let input =
            parse("AAAA
                   BBCD
                   BBCC
                   EEEC");

        let blocks = find_blocks(&input);
        assert_eq!(blocks.len(), 5);

        assert_eq!(blocks[0].id, 'A');
        assert_eq!(num_sides(&input, &blocks[0]), 4);

        assert_eq!(blocks[1].id, 'B');
        assert_eq!(num_sides(&input, &blocks[1]), 4);

        assert_eq!(blocks[2].id, 'C');
        assert_eq!(num_sides(&input, &blocks[2]), 8);

        assert_eq!(blocks[3].id, 'D');
        assert_eq!(num_sides(&input, &blocks[3]), 4);

        assert_eq!(blocks[4].id, 'E');
        assert_eq!(num_sides(&input, &blocks[4]), 4);
    }

    #[test]
    fn test_numsides_e_shape() {
        let input =
            parse("EEEEE
                   EXXXX
                   EEEEE
                   EXXXX
                   EEEEE");

        let blocks = find_blocks(&input);
        assert_eq!(blocks.len(), 3);

        assert_eq!(blocks[0].id, 'E');
        assert_eq!(num_sides(&input, &blocks[0]), 12);

        assert_eq!(blocks[1].id, 'X');
        assert_eq!(num_sides(&input, &blocks[1]), 4);

        assert_eq!(blocks[2].id, 'X');
        assert_eq!(num_sides(&input, &blocks[2]), 4);
    }

    #[test]
    fn test_numsides_third_map() {
        let input =
            parse("AAAAAA
                   AAABBA
                   AAABBA
                   ABBAAA
                   ABBAAA
                   AAAAAA");

        let blocks = find_blocks(&input);
        assert_eq!(blocks.len(), 3);

        assert_eq!(blocks[0].id, 'A');
        assert_eq!(num_sides(&input, &blocks[0]), 12);

        assert_eq!(blocks[1].id, 'B');
        assert_eq!(num_sides(&input, &blocks[1]), 4);

        assert_eq!(blocks[2].id, 'B');
        assert_eq!(num_sides(&input, &blocks[2]), 4);
    }

    #[test]
    fn test_part2() {
        let input =
            parse("RRRRIICCFF
                   RRRRIICCCF
                   VVRRRCCFFF
                   VVRCCCJFFF
                   VVVVCJJCFE
                   VVIVCCJJEE
                   VVIIICJJEE
                   MIIIIIJJEE
                   MIIISIJEEE
                   MMMISSJEEE");
        assert_eq!(part2(&input), 1206);
    }
}

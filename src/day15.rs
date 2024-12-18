#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::{thread, time};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    MoveN,
    MoveS,
    MoveW,
    MoveE,
}

use Instruction::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridElem {
    Empty,
    Wall,
    SmallBox,
    BoxLeft,
    BoxRight,
    Robot
}

use GridElem::*;

type Grid = Vec<Vec<GridElem>>;
type Instructions = Vec<Instruction>;
type Input = (Grid, Instructions);

#[aoc_generator(day15, part1)]
fn parse(input: &str) -> Input {
    let mut lit = input.lines();
    let mut grid = Vec::new();
    
    loop {
        if let Some(line) = lit.next() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let row = line.chars().map(|c| match c {
                '#' => Wall,
                'O' => SmallBox,
                '[' => BoxLeft,
                ']' => BoxRight,
                '@' => Robot,
                _   => Empty
            }).collect();

            grid.push(row);
        } else {
            break;
        };
    }

    let mut instructions = Vec::new();
    while let Some(line) = lit.next() {
        let line = line.trim();
        line.chars().map(|c| match c {
            '>' => MoveE,
            '<' => MoveW,
            '^' => MoveN,
            'v' => MoveS,
            _   => panic!("unexpected input!"),
        }).for_each(|i| instructions.push(i));
    }
    
    (grid, instructions)
}

#[aoc_generator(day15, part2)]
fn parse_double(input: &str) -> Input {
    let mut lit = input.lines();
    let mut grid = Vec::new();
    
    loop {
        if let Some(line) = lit.next() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let row = line.chars().flat_map(|c| match c {
                '#' => vec![Wall, Wall],
                'O' => vec![BoxLeft, BoxRight],
                '@' => vec![Robot, Empty],
                _   => vec![Empty, Empty]
            }).collect();

            grid.push(row);
        } else {
            break;
        };
    }

    let mut instructions = Vec::new();
    while let Some(line) = lit.next() {
        let line = line.trim();
        line.chars().map(|c| match c {
            '>' => MoveE,
            '<' => MoveW,
            '^' => MoveN,
            'v' => MoveS,
            _   => panic!("unexpected input!"),
        }).for_each(|i| instructions.push(i));
    }
    
    (grid, instructions)
}

fn format_elem(elem: &GridElem) -> char{
    match elem {
        Wall => '#',
        SmallBox => 'O',
        BoxLeft => '[',
        BoxRight => ']',
        Robot => '@',
        Empty => '.',
    }
}
fn print_grid(grid: &Grid) {
    for i in 0 .. grid.len() {
        for j in 0 .. grid[i].len() {
            print!("{}", format_elem(&grid[i][j]));
        }
        println!()
    }
}

fn find_robot(grid: &Grid) -> Option<(usize, usize)> {
    for i in 0 .. grid.len() {
        for j in 0 .. grid[i].len() {
            if grid[i][j] == Robot {
                return Some((i, j));
            }
        }
    }
    None
}

fn target_pos((i, j): (usize, usize), instruction: &Instruction) -> (usize, usize){
    match instruction {
        MoveE => (i, j + 1),
        MoveN => (i - 1, j),
        MoveS => (i + 1, j),
        MoveW => (i, j - 1),
    }
}

fn can_move_to(grid: &Grid, pos: (usize, usize), instruction: &Instruction) -> bool {
    let (i, j) = pos;
    match grid[i][j] {
        Wall => false,
        Empty => true,
        Robot | SmallBox => can_move_to(grid, target_pos(pos, instruction), instruction),
        BoxLeft => {
            if let MoveN | MoveS = instruction {
                let tpos_left = target_pos(pos, instruction);
                let tpos_right = target_pos((i, j+1), instruction);
                can_move_to(grid, tpos_left, instruction) && can_move_to(grid, tpos_right, instruction)
            } else {
                can_move_to(grid, target_pos(pos, instruction), instruction)
            }
            
        },
        BoxRight => {
            if let MoveN | MoveS = instruction {
                let tpos_left = target_pos((i, j-1), instruction);
                let tpos_right = target_pos(pos, instruction);
                can_move_to(grid, tpos_left, instruction) && can_move_to(grid, tpos_right, instruction)
            } else {
                can_move_to(grid, target_pos(pos, instruction), instruction)
            }
        }
    }
}

fn move_elem(grid: &mut Grid, pos: (usize, usize), instruction: &Instruction) -> (usize, usize) {
    let (i, j) = pos;

    match grid[i][j] {
        // Walls and empty space cannot move, return the same
        Wall | Empty => (i, j),

        BoxLeft => {
            if let MoveN | MoveS = instruction {
                let tpos_left = target_pos(pos, instruction);
                let tpos_right = target_pos((i, j+1), instruction);

                if can_move_to(grid, tpos_left, instruction) && can_move_to(grid, tpos_right, instruction) {
                    move_elem(grid, tpos_left, instruction);
                    move_elem(grid, tpos_right, instruction);

                    grid[tpos_left.0][tpos_left.1] = grid[i][j];
                    grid[i][j] = Empty;

                    grid[tpos_right.0][tpos_right.1] = grid[i][j+1];
                    grid[i][j+1] = Empty;
                    tpos_left
                } else {
                    pos
                }
            } else {
                let tpos = target_pos(pos, instruction);

                // Try to move the target position
                move_elem(grid, tpos, instruction);
                
                // Check if target pos is empty
                let (ti, tj) = tpos;
                if grid[ti][tj] == Empty {
                    grid[ti][tj] = grid[i][j];
                    grid[i][j] = Empty;
                    tpos
                } else {
                    pos
                }
            }
            
        },

        BoxRight => {
            if let MoveN | MoveS = instruction {
                let tpos_left = target_pos((i, j-1), instruction);
                let tpos_right = target_pos(pos, instruction);

                if can_move_to(grid, tpos_left, instruction) && can_move_to(grid, tpos_right, instruction) {
                    move_elem(grid, tpos_left, instruction);
                    move_elem(grid, tpos_right, instruction);

                    grid[tpos_left.0][tpos_left.1] = grid[i][j-1];
                    grid[i][j-1] = Empty;

                    grid[tpos_right.0][tpos_right.1] = grid[i][j];
                    grid[i][j] = Empty;
                    tpos_right
                } else {
                    pos
                }
            } else {
                let tpos = target_pos(pos, instruction);

                // Try to move the target position
                move_elem(grid, tpos, instruction);
                
                // Check if target pos is empty
                let (ti, tj) = tpos;
                if grid[ti][tj] == Empty {
                    grid[ti][tj] = grid[i][j];
                    grid[i][j] = Empty;
                    tpos
                } else {
                    pos
                }
            }
            
        },
        
        Robot | SmallBox => {
            let tpos = target_pos(pos, instruction);

            // Try to move the target position
            move_elem(grid, tpos, instruction);
            
            // Check if target pos is empty
            let (ti, tj) = tpos;
            if grid[ti][tj] == Empty {
                grid[ti][tj] = grid[i][j];
                grid[i][j] = Empty;
                tpos
            } else {
                pos
            }
        }
    }
}

fn score(grid: &Grid) -> u32 {
    let mut result = 0;
    for i in 0 .. grid.len() {
        for j in 0 .. grid[i].len() {
            if grid[i][j] == SmallBox || grid[i][j] == BoxLeft {
                let i = i as u32;
                let j = j as u32;
                result += 100 * i + j
            }
        }
    }
    result
}

#[aoc(day15, part1)]
fn part1((grid, instructions): &Input) -> u32 {
    let mut grid = grid.clone();
    let mut robot = find_robot(&grid).unwrap();
    
    for i in instructions {
        robot = move_elem(&mut grid, robot, i);
    }
    score(&grid)
}

// #[aoc(day15, part1)]
fn part1_vis((grid, instructions): &Input) -> u32 {
    let mut grid = grid.clone();
    let mut robot = find_robot(&grid).unwrap();

    print!("\x1Bc");
    println!("INIT");
    print_grid(&grid);
    thread::sleep(time::Duration::from_millis(250));
    
    for i in instructions {
        robot = move_elem(&mut grid, robot, i);
        print!("\x1Bc");
        println!(" {:?}", i);
        print_grid(&grid);
        thread::sleep(time::Duration::from_millis(250));
    }

    score(&grid)
}

#[aoc(day15, part2)]
fn part2((grid, instructions): &Input) -> u32 {
    let mut grid = grid.clone();
    let mut robot = find_robot(&grid).unwrap();
    
    for i in instructions {
        robot = move_elem(&mut grid, robot, i);
    }

    print_grid(&grid);
    
    score(&grid)
}

// #[aoc(day15, part2)]
fn part2_vis((grid, instructions): &Input) -> u32 {
    let mut grid = grid.clone();
    let mut robot = find_robot(&grid).unwrap();

    print!("\x1Bc");
    println!("INIT");
    print_grid(&grid);
    thread::sleep(time::Duration::from_millis(200));
    
    for idx in 0 .. instructions.len() {
        let i = instructions[idx];
        robot = move_elem(&mut grid, robot, &i);
        print!("\x1Bc");
        println!(" {}/{} {:?}", idx + 1, instructions.len(), i);
        print_grid(&grid);
        thread::sleep(time::Duration::from_millis(200));
    }

    score(&grid)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (grid, instructions) = parse(
            "########
             #..O.O.#
             ##@.O..#
             #...O..#
             #.#.O..#
             #...O..#
             #......#
             ########
             
             <^^>>>vv<v>>v<<
             <^^>>>vv<v>>v<<");

        assert_eq!(grid.len(), 8);
        assert_eq!(grid[0].len(), 8);
        assert_eq!(grid[0][0], Wall);
        assert_eq!(grid[1][1], Empty);
        assert_eq!(grid[1][3], SmallBox);
        assert_eq!(grid[2][2], Robot);

        assert_eq!(instructions.len(), 30);
        assert_eq!(instructions[0], MoveW);
        assert_eq!(instructions[1], MoveN);
        assert_eq!(instructions[3], MoveE);
        assert_eq!(instructions[6], MoveS);
    }

    #[test]
    fn test_move_simple() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #.@.O..#
             #...O..#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveN);
        assert_eq!(pos, (1, 2));
        assert_eq!(grid[2][2], Empty);
        assert_eq!(grid[1][2], Robot);

        pos = move_elem(&mut grid, pos, &MoveS);
        assert_eq!(pos, (2, 2));
        assert_eq!(grid[1][2], Empty);
        assert_eq!(grid[2][2], Robot);

        pos = move_elem(&mut grid, pos, &MoveE);
        assert_eq!(pos, (2, 3));
        assert_eq!(grid[2][2], Empty);
        assert_eq!(grid[2][3], Robot);

        pos = move_elem(&mut grid, pos, &MoveW);
        assert_eq!(pos, (2, 2));
        assert_eq!(grid[2][3], Empty);
        assert_eq!(grid[2][2], Robot);
    }

    #[test]
    fn test_move_push_1() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #...O@.#
             #......#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveW);
        print_grid(&grid);
        assert_eq!(pos, (2, 4));
        assert_eq!(grid[2][4], Robot);
        assert_eq!(grid[2][3], SmallBox);
    }

    #[test]
    fn test_move_push_big_1() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #..[]@.#
             #......#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveW);
        print_grid(&grid);
        assert_eq!(pos, (2, 4));
        assert_eq!(grid[2][2], BoxLeft);
        assert_eq!(grid[2][3], BoxRight);
        assert_eq!(grid[2][4], Robot);
    }
    #[test]
    fn test_move_push_big_2() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #.@[]..#
             #......#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveE);
        print_grid(&grid);
        assert_eq!(pos, (2, 3));
        assert_eq!(grid[2][3], Robot);
        assert_eq!(grid[2][4], BoxLeft);
        assert_eq!(grid[2][5], BoxRight);
    }

    #[test]
    fn test_move_push_big_3() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #..[]..#
             #..@...#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveN);
        print_grid(&grid);
        assert_eq!(pos, (2, 3));
        assert_eq!(grid[2][3], Robot);
        assert_eq!(grid[1][3], BoxLeft);
        assert_eq!(grid[1][4], BoxRight);

        pos = move_elem(&mut grid, pos, &MoveN);
        assert_eq!(pos, (2, 3));
        assert_eq!(grid[2][3], Robot);
        assert_eq!(grid[1][3], BoxLeft);
        assert_eq!(grid[1][4], BoxRight);
    }

    #[test]
    fn test_move_push_big_4() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #..@...#
             #..[]..#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveS);
        print_grid(&grid);
        assert_eq!(pos, (3, 3));
        assert_eq!(grid[3][3], Robot);
        assert_eq!(grid[4][3], BoxLeft);
        assert_eq!(grid[4][4], BoxRight);

        pos = move_elem(&mut grid, pos, &MoveS);
        assert_eq!(pos, (3, 3));
        assert_eq!(grid[3][3], Robot);
        assert_eq!(grid[4][3], BoxLeft);
        assert_eq!(grid[4][4], BoxRight);
    }

    #[test]
    fn test_move_push_big_chain_1() {
        let (mut grid, instructions) = parse(
            "########
             #..@...#
             #..[]..#
             #.[][].#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveS);
        print_grid(&grid);
        assert_eq!(pos, (2, 3));
        assert_eq!(grid[2][3], Robot);
        assert_eq!(grid[3][3], BoxLeft);
        assert_eq!(grid[3][4], BoxRight);
        assert_eq!(grid[4][2], BoxLeft);
        assert_eq!(grid[4][3], BoxRight);
        assert_eq!(grid[4][4], BoxLeft);
        assert_eq!(grid[4][5], BoxRight);

        pos = move_elem(&mut grid, pos, &MoveS);
        print_grid(&grid);
        assert_eq!(pos, (2, 3));
        assert_eq!(grid[2][3], Robot);
        assert_eq!(grid[3][3], BoxLeft);
        assert_eq!(grid[3][4], BoxRight);
        assert_eq!(grid[4][2], BoxLeft);
        assert_eq!(grid[4][3], BoxRight);
        assert_eq!(grid[4][4], BoxLeft);
        assert_eq!(grid[4][5], BoxRight);
    }

    #[test]
    fn test_move_push_bug_edge_case() {
        let (mut grid, instructions) = parse(
            "########
             #..@...#
             #..[]..#
             #..[]..#
             #...[].#
             #..[]..#
             ########");

        print_grid(&grid);
        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveS);
        print_grid(&grid);
        assert_eq!(pos, (1, 3));
        assert_eq!(grid[1][3], Robot);
        // assert_eq!(grid[3][3], BoxLeft);
        // assert_eq!(grid[3][4], BoxRight);
        // assert_eq!(grid[4][2], BoxLeft);
        // assert_eq!(grid[4][3], BoxRight);
        // assert_eq!(grid[4][4], BoxLeft);
        // assert_eq!(grid[4][5], BoxRight);
    }

    #[test]
    fn test_move_push_big_chain_2() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #.[][].#
             #..[]..#
             #..@...#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveN);
        print_grid(&grid);
        assert_eq!(pos, (3, 3));
        assert_eq!(grid[3][3], Robot);
        assert_eq!(grid[2][3], BoxLeft);
        assert_eq!(grid[2][4], BoxRight);
        assert_eq!(grid[1][2], BoxLeft);
        assert_eq!(grid[1][3], BoxRight);
        assert_eq!(grid[1][4], BoxLeft);
        assert_eq!(grid[1][5], BoxRight);

        pos = move_elem(&mut grid, pos, &MoveN);
        print_grid(&grid);
        assert_eq!(pos, (3, 3));
        assert_eq!(grid[3][3], Robot);
        assert_eq!(grid[2][3], BoxLeft);
        assert_eq!(grid[2][4], BoxRight);
        assert_eq!(grid[1][2], BoxLeft);
        assert_eq!(grid[1][3], BoxRight);
        assert_eq!(grid[1][4], BoxLeft);
        assert_eq!(grid[1][5], BoxRight);
    }

    #[test]
    fn test_move_push_3() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #.OOO@.#
             #......#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveW);
        print_grid(&grid);
        assert_eq!(pos, (2, 4));
        assert_eq!(grid[2][4], Robot);
        assert_eq!(grid[2][3], SmallBox);
        assert_eq!(grid[2][2], SmallBox);
        assert_eq!(grid[2][1], SmallBox);
    }

    #[test]
    fn test_move_push_block() {
        let (mut grid, instructions) = parse(
            "########
             #......#
             #OOOO@.#
             #......#
             #......#
             ########");

        let mut pos = find_robot(&grid).unwrap();
        pos = move_elem(&mut grid, pos, &MoveW);
        print_grid(&grid);
        assert_eq!(pos, (2, 5));
        assert_eq!(grid[2][5], Robot);
        assert_eq!(grid[2][4], SmallBox);
        assert_eq!(grid[2][3], SmallBox);
        assert_eq!(grid[2][2], SmallBox);
        assert_eq!(grid[2][1], SmallBox);
    }

    #[test]
    fn test_example_1() {
        let (mut grid, instructions) = parse(
            "########
             #..O.O.#
             ##@.O..#
             #...O..#
             #.#.O..#
             #...O..#
             #......#
             ########
             
             <^^>>>vv<v>>v<<"
        );

        let mut robot = find_robot(&grid).unwrap();
        for i in instructions {
            robot = move_elem(&mut grid, robot, &i);
            print_grid(&grid);
        }
        assert_eq!(robot, (4, 4));
        assert_eq!(grid[1][5], SmallBox);
        assert_eq!(grid[1][6], SmallBox);
        assert_eq!(grid[3][6], SmallBox);
        assert_eq!(grid[4][3], SmallBox);
        assert_eq!(grid[5][4], SmallBox);
        assert_eq!(grid[6][4], SmallBox);
    }

    #[test]
    fn test_score() {
        let (grid, _) = parse(
            "########
             #....OO#
             ##.....#
             #.....O#
             #.#O@..#
             #...O..#
             #...O..#
             ########"
        );
        assert_eq!(score(&grid), 2028);
    }

    #[test]
    fn test_part1_small() {
        let input = parse(
            "########
             #..O.O.#
             ##@.O..#
             #...O..#
             #.#.O..#
             #...O..#
             #......#
             ########
             
             <^^>>>vv<v>>v<<"
        );
        assert_eq!(part1(&input), 2028);
    }
    
    #[test]
    fn test_part1_large() {
        let input = parse(
            "##########
             #..O..O.O#
             #......O.#
             #.OO..O.O#
             #..O@..O.#
             #O#..O...#
             #O..O..O.#
             #.OO.O.OO#
             #....O...#
             ##########
             
             <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
             vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
             ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
             <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
             ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
             ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
             >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
             <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
             ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
             v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
        );


        assert_eq!(part1(&input), 10092);
    }

    #[test]
    fn test_score_scaled() {
        let (grid, _) = parse(
            "####################
             ##[].......[].[][]##
             ##[]...........[].##
             ##[]........[][][]##
             ##[]......[]....[]##
             ##..##......[]....##
             ##..[]............##
             ##..@......[].[][]##
             ##......[][]..[]..##
             ####################"
        );
        assert_eq!(score(&grid), 9021);
    }

    #[test]
    fn test_part2_large() {
        let input = parse_double(
            "##########
             #..O..O.O#
             #......O.#
             #.OO..O.O#
             #..O@..O.#
             #O#..O...#
             #O..O..O.#
             #.OO.O.OO#
             #....O...#
             ##########
             
             <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
             vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
             ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
             <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
             ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
             ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
             >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
             <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
             ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
             v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
        );


        assert_eq!(part2(&input), 9021);
    }
}

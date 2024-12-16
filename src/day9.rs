#![allow(unused_variables, unused_mut, dead_code, unused_comparisons)]

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Entry {
    Empty,
    Block(i32),
}

use Entry::*;

type Input = Vec<Entry>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    let mut result = Vec::new();
    let mut block = true;
    let mut id = 0;
    input
        .chars()
        .for_each(|c| {
            let val = c as i32 - 0x30;

            if block {
                for _ in 0..val {
                    result.push(Block(id))
                }
                id += 1
            } else {
                for _ in 0..val {
                    result.push(Empty)
                }
            }
            
            block = !block
        });

    result
}

fn print_state(input: &Input) {
    for e in input {
        if let Block(id) = e {
            print!("|{:?}|", id);
        } else {
            print!("|-|");
        }
    }
    println!();
}

fn next_free(input: &Input, from_idx: usize) -> usize {
    let mut i = from_idx;

    while i < input.len() {
        match input[i] {
            Empty => break,
            _ => i += 1,
        }
    }
    return i;
}

fn next_filled(input: &Input, from_idx: usize) -> usize {
    let mut i = from_idx;

    while i >= 0 {
        match input[i] {
            Block(_) => break,
            _ => i -= 1,
        }
    }
    return i;
}

// Finds the next file block
fn next_file_block(input: &Input, from_idx: usize) -> Option<(usize, usize)> {
    let st_i = next_filled(input, from_idx);
    let mut idx = st_i;

    while idx > 0 && input[st_i] == input[idx] {
        idx -= 1;
    }
    if idx == 0 {
        Some((0, st_i + 1))
    } else {
        Some((idx + 1, st_i - idx))
    }
}

// Finds the first empty block with `size` size
fn find_empty_block(input: &Input, size: usize, max_idx: usize) -> Option<usize> {
    let mut cur_st = 0;
    let mut empty_found = false;
    
    for i in 0 ..= max_idx {
        if empty_found && Empty != input[i]{
            if i - cur_st >= size {
                return Some(cur_st);
            } else {
                empty_found = false;
            }
        } else if !empty_found && Empty == input[i] {
            empty_found = true;
            cur_st = i;
            //println!("Empty found {cur_st}");
        }
    }

    // println!("{empty_found} {max_idx} {cur_st} {}", max_idx - cur_st);
    if empty_found && max_idx - cur_st + 1 >= size {
        Some(cur_st)
    } else {
        None
    }
}

fn move_file_block(input: &mut Input, from_idx: usize, to_idx: usize, size: usize) {
    for i in 0 .. size {
        let prev = input[from_idx + i];
        input[from_idx + i] = input[to_idx + i];
        input[to_idx + i] = prev;
    }
}

fn checksum(input: &Input) -> i64 {
    let mut result = 0;
    for (idx, elem) in input.iter().enumerate() {
        match elem {
            Block(id) => result += idx as i64 * (*id as i64),
            _ => ()
        }
    }
    result
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> i64 {
    // print_state(input);

    let mut result = input.clone();
    let mut free_idx = next_free(input, 0);
    let mut cur_block = next_filled(input, input.len() - 1);

    while free_idx < cur_block {
        result[free_idx] = result[cur_block];
        result[cur_block] = Empty;

        free_idx = next_free(&result, free_idx);
        cur_block = next_filled(&result, cur_block);

        // print_state(&result);
    }

    // print_state(&result);
    return checksum(&result);
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> i64 {
    let mut result = input.clone();
    let mut idx = input.len() - 1;

    while idx > 0 {
        if let Some((block_idx, block_size)) = next_file_block(&result, idx) {
            // println!("Next: {:?} {} {}", result[block_idx], block_idx, block_size);
            if let Some(target_idx) = find_empty_block(&result, block_size, block_idx) {
                // println!("MOVE {}", target_idx);
                move_file_block(&mut result, block_idx, target_idx, block_size);
            } else {
                // println!("NOT MOVE");
            }
            if block_idx > 0 {
                idx = block_idx - 1
            } else {
                break;
            }
        } else {
            break;
        }

        // print_state(&result);
    }

    // print_state(&result);
    
    return checksum(&result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let input = "12345";
        let result = parse(input);
        assert_eq!(
            result,
            vec![
                Block(0), Empty, Empty, Block(1), Block(1), Block(1),
                Empty, Empty, Empty, Empty, Block(2), Block(2), Block(2), Block(2), Block(2),
            ]
        );

        let input = "2333133121414131402";
        let result = parse(input);
        assert_eq!(result.len(), 42);
        assert_eq!(result[0], Block(0));
        assert_eq!(result[41], Block(9));
    }

    #[test]
    fn test_part1() {
        let input = parse("2333133121414131402");
         assert_eq!(part1(&input), 1928);

        let input = parse("929292");
        assert_eq!(part1(&input), 495);

        let input = parse("54321");
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_next_block() {
        let input = vec![
            Block(0), Empty, Empty, Block(1), Block(1),
            Block(1), Empty, Empty, Empty, Empty,
            Block(2), Block(2), Block(2), Block(2), Block(2),
            Empty, Block(3), Block(3), Block(4), Block(4),
            Empty, Empty, Block(5),
        ];

        assert_eq!(next_file_block(&input, 1),  Some((0, 1)));
        assert_eq!(next_file_block(&input, 9),  Some((3, 3)));
        assert_eq!(next_file_block(&input, 14), Some((10, 5)));
        assert_eq!(next_file_block(&input, 15), Some((10, 5)));
        assert_eq!(next_file_block(&input, 17), Some((16, 2)));
        assert_eq!(next_file_block(&input, 21), Some((18, 2)));
        assert_eq!(next_file_block(&input, 22), Some((22, 1)));
    }

    #[test]
    fn test_empty_block() {
        let input = vec![
            Block(0), Empty, Empty, Block(1), Block(1),
            Block(1), Empty, Empty, Empty, Empty,
            Empty, Block(2), Block(2), Block(2), Block(2),
            Empty, Block(3), Block(3), Block(4), Block(4),
            Empty, Empty, Block(5),
        ];

        assert_eq!(find_empty_block(&input, 2, 22),  Some(1));
        assert_eq!(find_empty_block(&input, 3, 8),  Some(6));
        assert_eq!(find_empty_block(&input, 3, 5),  None);
        assert_eq!(find_empty_block(&input, 10, 22),  None);
        assert_eq!(find_empty_block(&input, 5, 22),  Some(6));
    }

    #[test]
    fn test_move_block() {
        let mut input = vec![
            Block(0), Empty, Empty, Block(1), Block(1),
            Block(1), Empty, Empty, Empty, Empty,
            Empty, Block(2), Block(2), Block(2), Block(2),
            Empty, Block(3), Block(3), Block(4), Block(4),
            Empty, Empty, Block(5),
        ];

        move_file_block(&mut input, 18, 1, 2);
        assert_eq!(input[1], Block(4));
        assert_eq!(input[2], Block(4));
        assert_eq!(input[18], Empty);
        assert_eq!(input[19], Empty);

        move_file_block(&mut input, 22, 6, 1);
        assert_eq!(input[6], Block(5));
        assert_eq!(input[22], Empty);

        move_file_block(&mut input, 16, 7, 2);
        assert_eq!(input[7], Block(3));
        assert_eq!(input[8], Block(3));
        assert_eq!(input[16], Empty);
    }

    #[test]
    fn test_part2() {
        let input = parse("2333133121414131402");
        assert_eq!(part2(&input), 2858);

        // let input = parse("929292");
        // assert_eq!(part1(&input), 495);

        // let input = parse("54321");
        // assert_eq!(part1(&input), 31);
    }
}

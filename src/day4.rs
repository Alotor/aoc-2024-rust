#![allow(unused_variables, unused_mut, dead_code)]

use crate::utils::get2d;

type Input = Vec<Vec<char>>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.trim().chars().collect()
        })
        .collect()
}

// Safely access the array of inputs
// fn get(input: &Input, i: i32, j: i32) -> &char {
//     if let Ok(i) = usize::try_from(i) {
//         if let Some(v2) = input.get(i) {
//             if let Ok(j) = usize::try_from(j) {
//                 v2.get(j).unwrap_or(&'.')
//             } else {
//                 &'.'
//             }
//         } else {
//             &'.'
//         }
//     } else {
//         &'.'
//     }
// }

fn letters(input: &Input, i: i32, j: i32, length: i32) -> Vec<String> {
    vec![
        (0..length).map(|a| get2d(input, i, j + a)).collect(),
        (0..length).map(|a| get2d(input, i + a, j)).collect(),
        (0..length).map(|a| get2d(input, i, j - a)).collect(),
        (0..length).map(|a| get2d(input, i - a, j)).collect(),

        (0..length).map(|a| get2d(input, i + a, j + a)).collect(),
        (0..length).map(|a| get2d(input, i - a, j + a)).collect(),
        (0..length).map(|a| get2d(input, i - a, j - a)).collect(),
        (0..length).map(|a| get2d(input, i + a, j - a)).collect(),
    ]
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> u32 {
    let mut result: u32 = 0;
    for i in 0 .. input.len() {
        for j in 0 .. input[0].len() {
            let letters = letters(input, i as i32, j as i32, 4);
            // println!("{i}, {j} = {letters:?}");
            result += letters.iter().filter(|s| *s == "XMAS").count() as u32;
        }
    }
    result
}

fn letters_2(input: &Input, i: i32, j: i32, length: i32) -> Vec<String> {
    let v1: Vec<&char> =
        vec![get2d(input, i - 1, j - 1),
             get2d(input, i, j),
             get2d(input, i + 1, j + 1),
        ];

    let v2 =
        vec![get2d(input, i - 1, j + 1),
             get2d(input, i, j),
             get2d(input, i + 1, j - 1)
        ];

    
    vec![String::from_iter(v1), String::from_iter(v2)]
}


#[aoc(day4, part2)]
fn part2(input: &Input) -> u32 {
    let mut result: u32 = 0;
    for i in 0 .. input.len() {
        for j in 0 .. input[0].len() {
            let letters = letters_2(input, i as i32, j as i32, 4);

            if (letters[0] == "MAS" || letters[0] == "SAM") && (letters[1] == "MAS" || letters[1] == "SAM"){
                // println!("{i}, {j} = {:?} {:?}", letters[0], letters[1]);
                result += 1
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator() {
        let input = parse("SMXXSM
                           SSAMXM
                           SAXMAA
                           XMASMS
                           XXSAMS");

        assert_eq!(input[0][0], 'S');
        assert_eq!(input[0][1], 'M');
        assert_eq!(input[0][2], 'X');

        assert_eq!(input[1][0], 'S');
        assert_eq!(input[1][1], 'S');
        assert_eq!(input[1][2], 'A');

        assert_eq!(input[4][3], 'A');
        assert_eq!(input[4][4], 'M');
        assert_eq!(input[4][5], 'S');
    }

    

    #[test]
    fn test_letters() {
        let input = parse("SMXXSM
                           SSAMXM
                           SAXMAA
                           XMASMS
                           XXSAMS");

        let out = letters(&input, 0, 0, 5);
        assert_eq!(out, vec![
            "SMXXS",
            "SSSXX",
            "S    ",
            "S    ",
            "SSXSM",
            "S    ",
            "S    ",
            "S    "]);
    }

    #[test]
    fn test_part1_a() {
        let input = parse("..X...
                           .SAMX.
                           .A..A.
                           XMAS.S
                           .X....");

        assert_eq!(part1(&input), 4);
    }
    
    #[test]
    fn test_part1_b() {
        let input = parse("MMMSXXMASM
                           MSAMXMSMSA
                           AMXSXMAAMM
                           MSAMASMSMX
                           XMASAMXAMM
                           XXAMMXXAMA
                           SMSMSASXSS
                           SAXAMASAAA
                           MAMMMXMMMM
                           MXMXAXMASX");

        assert_eq!(part1(&input), 18);
    }
    
    #[test]
    fn test_part2_b() {
        let input = parse("MMMSXXMASM
                           MSAMXMSMSA
                           AMXSXMAAMM
                           MSAMASMSMX
                           XMASAMXAMM
                           XXAMMXXAMA
                           SMSMSASXSS
                           SAXAMASAAA
                           MAMMMXMMMM
                           MXMXAXMASX");

        assert_eq!(part2(&input), 9);
    }
}

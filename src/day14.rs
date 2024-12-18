#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use regex::Regex;
use std::collections::HashMap;
use std::{thread, time};


#[derive(Debug, PartialEq, Clone)]
struct Entry {
    pos: (u32, u32),
    vel: (i32, i32),
}

type Input = Vec<Entry>;

#[aoc_generator(day14)]
fn parse(input: &str) -> Input {
    let entry_re = Regex::new(r"p\=(\d+),(\d+) v\=(-?\d+),(-?\d+)").unwrap();

    input
        .lines()
        .map(|l| {
            let cc = entry_re.captures(l).unwrap();
            let px = cc[1].parse::<u32>().unwrap();
            let py = cc[2].parse::<u32>().unwrap();
            let vx = cc[3].parse::<i32>().unwrap();
            let vy = cc[4].parse::<i32>().unwrap();
            
            Entry { pos: (px,py), vel: (vx,vy) }
        })
        .collect()
}

fn state_map(input: &Input) -> HashMap::<&(u32, u32), Vec<&Entry>> {
    let mut locations = HashMap::<&(u32, u32), Vec<&Entry>>::new();

    for entry in input {
        if let Some(entries) = locations.get_mut(&entry.pos) {
            entries.push(entry);
        } else {
            locations.insert(&entry.pos, Vec::from([entry]));    
        }
    }

    locations
}

fn print_state(input: &Input, width: u32, height: u32) {
    let locations = state_map(input);

    for y in 0 .. height {
        for x in 0 .. width {
            if let Some(entries) = locations.get(&(x, y)) {
                // print!("{}", entries.len());
                print!("\u{25A0}")
            //} else if x == width / 2 || y == height / 2 {
            //    print!(" ");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn sim_entry(entry: &mut Entry, width: u32, height: u32) {
    let mut x = entry.pos.0 as i32 + entry.vel.0;
    let mut y = entry.pos.1 as i32 + entry.vel.1;

    x = x.rem_euclid(width as i32);
    y = y.rem_euclid(height as i32);

    entry.pos.0 = x as u32;
    entry.pos.1 = y as u32;
}

fn sim_step(input: &mut Input, width: u32, height: u32) {
    for i in 0 .. input.len() {
        sim_entry(&mut input[i], width, height);
    }
}

fn simulate(input: &mut Input, width: u32, height: u32, iterations: u32) {
    for i in 0 .. iterations {
        sim_step(input, width, height);
    }
}

fn qx(input: &Input, xmin: u32, xmax: u32, ymin: u32, ymax: u32) -> u32 {
    input
        .iter()
        .filter(|entry| {
            entry.pos.0 >= xmin &&
            entry.pos.0 <= xmax &&
            entry.pos.1 >= ymin &&
            entry.pos.1 <= ymax
        })
        .count() as u32
}

fn q1(input: &Input, width: u32, height: u32) -> u32 {
    let xmin = 0;
    let xmax = width / 2 - 1;
    let ymin = 0;
    let ymax = height / 2 - 1;
    qx(input, xmin, xmax, ymin, ymax)
}

fn q2(input: &Input, width: u32, height: u32) -> u32 {
    let xmin = width / 2 + 1;
    let xmax = width - 1;
    let ymin = 0;
    let ymax = height / 2 - 1;
    qx(input, xmin, xmax, ymin, ymax)
}

fn q3(input: &Input, width: u32, height: u32) -> u32 {
    let xmin = 0;
    let xmax = width / 2 - 1;
    let ymin = height / 2 + 1;
    let ymax = height - 1;
    qx(input, xmin, xmax, ymin, ymax)
}

fn q4(input: &Input, width: u32, height: u32) -> u32 {
    let xmin = width / 2 + 1;
    let xmax = width - 1;
    let ymin = height / 2 + 1;
    let ymax = height - 1;
    qx(input, xmin, xmax, ymin, ymax)
}

fn safety_factor(input: &Input, width: u32, height: u32) -> u32 {
    q1(input, width, height) *
        q2(input, width, height) *
        q3(input, width, height) *
        q4(input, width, height)
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> u32 {
    let width: u32 = 101;
    let height: u32  = 103;
    
    let mut state: Input = input.clone();
    //print_state(&state, width, height);
    simulate(&mut state, width, height, 100);
    //print_state(&input, width, height);

    safety_factor(&state, width, height)
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> u32 {
    let width: u32 = 101;
    let height: u32  = 103;
    let mut state: Input = input.clone();

    // We can use the safety_factor as an heuristic to search for
    // the drawing.
    let mut sf = Vec::<(u32, u32)>::new();
    for step in 1 ..= 50000 {
        sim_step(&mut state, width, height);
        let safety_factor = safety_factor(&state, width, height);
        sf.push((safety_factor, step));
    }

    let min = sf.iter().min().unwrap();

    // Uncomment these lines to print the image
    // let mut state: Input = input.clone();
    // simulate(&mut state, width, height, min.1);
    // print_state(&state, width, height);
    

    // Code for rendering the steps
    /*
    for step in 1 ..= 50000 {
        // simulate(&mut state, width, height, step);

        sim_step(&mut state, width, height);
        
        let safety_factor = safety_factor(&state, width, height);

        if safety_factor < 97000000 {
            print!("\x1Bc");
            println!("STEP: {step} : {safety_factor}");
            print_state(&state, width, height);
            thread::sleep(time::Duration::from_millis(10000));
        }
    
        // thread::sleep(time::Duration::from_millis(250));
    }
    */

    min.1
}

// #[aoc(day14, part2)]
// fn part2(input: &Input) -> u32 {
//     todo!()
// }


#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: u32 = 11;
    const HEIGHT: u32 = 7;

    #[test]
    fn test_parse() {
        let input = parse(
            "p=0,4 v=3,-3
             p=6,3 v=-1,-3
             p=10,3 v=-1,2
             p=2,0 v=2,-1
             p=0,0 v=1,3
             p=3,0 v=-2,-2
             p=7,6 v=-1,-3
             p=3,0 v=-1,-2
             p=9,3 v=2,3
             p=7,3 v=-1,2
             p=2,4 v=2,-3
             p=9,5 v=-3,-3"
        );
        assert_eq!(input, vec![
            Entry { pos: (0, 4), vel: (3, -3) },
            Entry { pos: (6, 3), vel: (-1, -3) },
            Entry { pos: (10, 3), vel: (-1, 2) },
            Entry { pos: (2, 0), vel: (2, -1) },
            Entry { pos: (0, 0), vel: (1, 3) },
            Entry { pos: (3, 0), vel: (-2, -2) },
            Entry { pos: (7, 6), vel: (-1, -3) },
            Entry { pos: (3, 0), vel: (-1, -2) },
            Entry { pos: (9, 3), vel: (2, 3) },
            Entry { pos: (7, 3), vel: (-1, 2) },
            Entry { pos: (2, 4), vel: (2, -3) },
            Entry { pos: (9, 5), vel: (-3, -3) },
        ]);
    }

    // #[test]
    // fn test_print() {
    //     let input = vec![
    //         Entry { pos: (3, 2), vel: (3, -3) },
    //         Entry { pos: (0, 4), vel: (3, -3) },
    //         Entry { pos: (0, 4), vel: (-1, -3) },
    //     ];
    //         
    //     print_state(&input, 11, 11);
    // }

    #[test]
    fn test_sim_step() {
        let mut input = vec![
            Entry { pos: (2, 4), vel: (2, -3) },
        ];

        print_state(&input, WIDTH, HEIGHT);

        sim_step(&mut input, WIDTH, HEIGHT);
        print_state(&input, WIDTH, HEIGHT);
        assert_eq!(input[0].pos, (4, 1));

        sim_step(&mut input, WIDTH, HEIGHT);
        print_state(&input, WIDTH, HEIGHT);
        assert_eq!(input[0].pos, (6, 5));

        sim_step(&mut input, WIDTH, HEIGHT);
        print_state(&input, WIDTH, HEIGHT);
        assert_eq!(input[0].pos, (8, 2));

        sim_step(&mut input, WIDTH, HEIGHT);
        print_state(&input, WIDTH, HEIGHT);
        assert_eq!(input[0].pos, (10, 6));
    }

    #[test]
    fn test_simulate() {
        let mut input = parse(
            "p=0,4 v=3,-3
             p=6,3 v=-1,-3
             p=10,3 v=-1,2
             p=2,0 v=2,-1
             p=0,0 v=1,3
             p=3,0 v=-2,-2
             p=7,6 v=-1,-3
             p=3,0 v=-1,-2
             p=9,3 v=2,3
             p=7,3 v=-1,2
             p=2,4 v=2,-3
             p=9,5 v=-3,-3"
        );


        print_state(&input, WIDTH, HEIGHT);
        simulate(&mut input, WIDTH, HEIGHT, 100);
        print_state(&input, WIDTH, HEIGHT);

        let locations = state_map(&input);
        assert_eq!(locations.get(&(6, 0)).unwrap().len(), 2);
        assert_eq!(locations.get(&(9, 0)).unwrap().len(), 1);
        assert_eq!(locations.get(&(0, 2)).unwrap().len(), 1);
        assert_eq!(locations.get(&(3, 5)).unwrap().len(), 1);
        assert_eq!(locations.get(&(4, 5)).unwrap().len(), 2);
    }

    #[test]
    fn test_safety_factor() {
        let mut input = parse(
            "p=0,4 v=3,-3
             p=6,3 v=-1,-3
             p=10,3 v=-1,2
             p=2,0 v=2,-1
             p=0,0 v=1,3
             p=3,0 v=-2,-2
             p=7,6 v=-1,-3
             p=3,0 v=-1,-2
             p=9,3 v=2,3
             p=7,3 v=-1,2
             p=2,4 v=2,-3
             p=9,5 v=-3,-3"
        );


        print_state(&input, WIDTH, HEIGHT);
        simulate(&mut input, WIDTH, HEIGHT, 100);
        print_state(&input, WIDTH, HEIGHT);

        assert_eq!(q1(&input, WIDTH, HEIGHT), 1);
        assert_eq!(q2(&input, WIDTH, HEIGHT), 3);
        assert_eq!(q3(&input, WIDTH, HEIGHT), 4);
        assert_eq!(q4(&input, WIDTH, HEIGHT), 1);
        assert_eq!(safety_factor(&input, WIDTH, HEIGHT), 12);
    }
}

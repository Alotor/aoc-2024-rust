#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashSet, HashMap};

type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let mut patterns = Vec::<String>::new();
    let mut out = Vec::<String>::new();

    let fline = lines.next().unwrap();

    for elem in fline.trim().split(", ") {
        patterns.push(String::from(elem));
    }

    // Skip line
    lines.next();

    while let Some(l) = lines.next() {
        out.push(l.trim().to_string());
    }

    (patterns, out)
}

fn has_pattern(patterns: &HashSet<String>, line: &String, start: usize, end: usize) -> bool {
    if start > line.len() - 1 || end > line.len() - 1 {
        false
    } else {
        patterns.contains(&line[start..end+1].to_string())
    }
}

fn search_patterns(patterns: &HashSet<String>, line: &String, max_pat_length: usize) -> bool {
    let mut cache = HashMap::<String, bool>::new();
    search_patterns_cache(patterns, line, max_pat_length, &mut cache)
}

fn search_patterns_cache(patterns: &HashSet<String>, line: &String, max_pat_length: usize, cache: &mut HashMap::<String, bool>) -> bool {
    fn search_patterns_rec(patterns: &HashSet<String>, line: &String, max_pat_length: usize, cache: &mut HashMap::<String, bool>, start: usize) -> bool {
        let line_str = line[start..].to_string();
        if cache.contains_key(&line_str) {
            return *cache.get(&line_str).unwrap();
        }

        if start == line.len(){
            return true;
        }
        
        let mut subpt = Vec::<usize>::new();

        for i in 0 .. max_pat_length {
            // let i = max_pat_length - i - 1;
            
            if has_pattern(patterns, &line, start, start+i) {
                subpt.push(start+i);
            }
        }

        let result = subpt
            .iter()
            .any(|end| {
                if end+1 == line.len() {
                    // println!("FOUND!");
                    true
                } else {
                    // println!("TRY {}-{}", &line[start..end+1], &line[end+1..]);
                    search_patterns_rec(patterns, line, max_pat_length, cache, end+1)
                }
            });

        cache.insert(line[start..].to_string(), result);
        
        result
    }

    // println!("Search {line}");
    
    search_patterns_rec(patterns, line, max_pat_length, cache, 0)
}

#[aoc(day19, part1)]
fn part1((patterns, lines): &Input) -> usize {

    let patterns = HashSet::<String>::from_iter(
        patterns.iter().map(|s| s.to_string())
    );

    let max_pat_len = patterns.iter().map(|s| s.len()).max().unwrap();
    let mut cache = HashMap::<String, bool>::new();

    let mut result = 0;
    for i in 0 .. lines.len() {
        let line = &lines[i];
        if search_patterns_cache(&patterns, &line, max_pat_len, &mut cache) {
            result += 1
        }
    }

    result
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (patterns, lines) = parse(
            "r, wr, b, g, bwu, rb, gb, br

             brwrr
             bggr
             gbbr
             rrbgbr
             ubwu
             bwurrg
             brgr
             bbrgwb"
        );
        assert_eq!(patterns, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert_eq!(lines, vec![
            "brwrr",
            "bggr",
            "gbbr",
            "rrbgbr",
            "ubwu",
            "bwurrg",
            "brgr",
            "bbrgwb",
        ]);
    }

    #[test]
    fn test_has_pattern() {
        let patterns = HashSet::from_iter(
            ["b", "brw", "rw"]
                .iter()
                .map(|s| s.to_string())
        );
        assert!(has_pattern(&patterns, &String::from("brwrr"), 0, 0));
        assert!(!has_pattern(&patterns, &String::from("brwrr"), 0, 1));
        assert!(has_pattern(&patterns, &String::from("brwrr"), 0, 2));
        assert!(!has_pattern(&patterns, &String::from("brwrr"), 1, 1));
        assert!(has_pattern(&patterns, &String::from("brwrr"), 1, 2));
    }

    
    #[test]
    fn test_search_patterns() {
        let patterns = HashSet::from_iter(
            ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].iter().map(|s| s.to_string())
        );
        
        assert!(search_patterns(&patterns, &String::from("brwrr"), 3));
        assert!(search_patterns(&patterns, &String::from("bggr"), 3));
        assert!(!search_patterns(&patterns, &String::from("ubwu"), 3));
        assert!(!search_patterns(&patterns, &String::from("bbrgwb"), 3));
    }

    #[test]
    fn test_part1() {
        let input = parse(
            "r, wr, b, g, bwu, rb, gb, br

             brwrr
             bggr
             gbbr
             rrbgbr
             ubwu
             bwurrg
             brgr
             bbrgwb"
        );
        assert_eq!(part1(&input), 6);
    }
}

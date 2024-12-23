#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashSet, HashMap};
use std::hash::{Hash, BuildHasher};


type Graph = HashMap<String, HashSet<String>>;
type Set<T> = HashSet<T>;

#[aoc_generator(day23)]
fn parse(input: &str) -> Graph {
    let mut result = HashMap::<String, Set<String>>::new();
    input
        .lines()
        .for_each(|l| {
            let p: Vec<_> = l.trim().split("-").collect();
            let p0 = p[0].to_string();
            let p1 = p[1].to_string();
            
            if result.contains_key(&p0) {
                result.get_mut(&p0).unwrap().insert(p1.clone());
            } else {
                result.insert(p0.clone(), Set::<String>::from([p1.clone()]));
            }

            if result.contains_key(&p1) {
                result.get_mut(&p1).unwrap().insert(p0.clone());
            } else {
                result.insert(p1.clone(), Set::<String>::from([p0.clone()]));
            }
        });
    result
        
}

fn find_cycles_start_nodes(graph: &Graph, max_level: usize, start: Vec<String>) -> Set<Vec<String>> {
    fn find_cycles_rec(graph: &Graph, prev: &String, node: &String, level: usize, visited: Set<String>) -> Vec<Set<String>> {
        let mut result = Vec::<Set<String>>::new();

        if level > 0 {
            for next in graph.get(node).unwrap_or(&Set::new()) {
                if next != prev {
                    let mut visited = visited.clone();

                    if visited.contains(next) {
                        result.push(visited);
                    } else {
                        visited.insert(next.clone());
                        let next_result = find_cycles_rec(graph, node, next, level - 1, visited);
                        next_result.iter().for_each(|e| result.push(e.clone()));
                    }
                }
            }
        }
        result
    }

    let mut result = Vec::<Set<String>>::new();
    let e = Set::new();
    for node in start {
        let children =  graph.get(&node).unwrap_or(&e);
        for next in children {
            let next_result = find_cycles_rec(&graph, &node, next, max_level - 1, Set::from([node.clone(), next.clone()]));
            next_result.iter().for_each(|e| result.push(e.clone()));
        }
        
    }

    let mut out = Set::<Vec<String>>::new();
    for e in result {
        let mut k: Vec<String> = e.iter().map(|e| e.clone()).collect();
        k.sort();
        out.insert(k);
    }
    
    out
}

fn find_cycles(graph: &Graph, max_level: usize) -> Set<Vec<String>> {
    let ks: Vec<String> = graph.keys().map(|s| s.clone()).collect();
    find_cycles_start_nodes(graph, max_level, ks)
}

#[aoc(day23, part1)]
fn part1(input: &Graph) -> u32 {
    let ks: Vec<String> = input.keys()
        .filter(|s| s.starts_with("t"))
        .map(|s| s.clone()).collect();

    let result = find_cycles_start_nodes(input, 3, ks);
    
    result.len() as u32
}

fn is_connected(graph: &Graph, s1: &String, s2: &String) -> bool {
    if let Some(next) = graph.get(s1) {
        next.contains(s2)
    } else {
        false
    }
}

fn stv(set: &Set<String>) -> Vec<String> {
    let mut k: Vec<String> = set.iter().map(|e| e.clone()).collect();
    k.sort();
    k
}

fn vts(v: &Vec<String>) -> Set<String> {
    Set::from_iter(v.iter().map(|s| s.clone()))
}

fn find_connected_nodes(graph: &Graph) -> Vec<String> {
    fn find_connected_nodes(
        graph: &Graph,
        cset: Set<String>,
        node: &String,
        connected: &mut Set::<Vec<String>>,
        disconnected: &mut Set::<Vec<String>>)
    {
        let e = Set::new();
        let children =  graph.get(node).unwrap_or(&e);

        for next in children {
            if !cset.contains(next) {
                let mut nset = cset.clone();
                nset.insert(next.clone());
                let nsetv = stv(&nset);

                if connected.contains(&nsetv) || disconnected.contains(&nsetv) {
                    continue;
                }
                
                if cset.iter().all(|c| is_connected(graph, c, next)) {
                    connected.insert(nsetv);
                    find_connected_nodes(graph, nset, next, connected, disconnected);
                } else {
                    disconnected.insert(nsetv);
                }
            }
        }
    }

    let mut connected = Set::<Vec<String>>::new();
    let mut disconnected = Set::<Vec<String>>::new();

    for node in graph.keys() {
        let cset = Set::<String>::from_iter([node.clone()]);
        find_connected_nodes(graph, cset, node, &mut connected, &mut disconnected);
    }

    let mut max = connected.iter().max_by(|a, b| {
        a.len().cmp(&b.len())
    }).unwrap().clone();

    max.sort();
    max
}

#[aoc(day23, part2)]
fn part2(input: &Graph) -> String {
    let result = find_connected_nodes(&input);
    result.join(",")
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> Graph {
        parse(
            "kh-tc
             qp-kh
             de-cg
             ka-co
             yn-aq
             qp-ub
             cg-tb
             vc-aq
             tb-ka
             wh-tc
             yn-cg
             kh-ub
             ta-co
             de-co
             tc-td
             tb-wq
             wh-td
             ta-ka
             td-qp
             aq-cg
             wq-ub
             ub-vc
             de-ta
             wq-aq
             wq-vc
             wh-yn
             ka-de
             kh-ta
             co-tc
             wh-qp
             tb-vc
             td-yn"
        )
    }
    
    #[test]
    fn test_parse() {
        let input = sample_graph();

        assert!(is_connected(&input, &String::from("kh"), &String::from("tc")));
        assert!(is_connected(&input, &String::from("tc"), &String::from("kh")));
        assert!(!is_connected(&input, &String::from("kh"), &String::from("td")));
        assert!(!is_connected(&input, &String::from("kh"), &String::from("ee")));
        assert!(!is_connected(&input, &String::from("ee"), &String::from("tc")));

        assert_eq!(
            input.get(&String::from("kh")).unwrap(),
            &HashSet::from([
                String::from("tc"),
                String::from("ub"),
                String::from("ta"),
                String::from("qp")
            ]));

        assert_eq!(
            input.get(&String::from("qp")).unwrap(),
            &HashSet::from([
                String::from("kh"),
                String::from("ub"),
                String::from("td"),
                String::from("wh")
            ]));

        assert_eq!(
            input.get(&String::from("td")).unwrap(),
            &HashSet::from([
                String::from("qp"),
                String::from("yn"),
                String::from("tc"),
                String::from("wh")
            ]));
    }

    #[test]
    fn test_cicle_3() {
        let input = sample_graph();
        let result = find_cycles(&input, 3);
        assert_eq!(result.len(), 12);
    }

    #[test]
    fn test_part1() {
        let input = sample_graph();
        let result = part1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_connected() {
        let input = sample_graph();
        let result = find_connected_nodes(&input);

        assert_eq!(result, vec![
            String::from("co"),
            String::from("de"),
            String::from("ka"),
            String::from("ta")
        ]);
    }

     #[test]
    fn test_connected_2() {
        let input = parse("g-a
        a-b
        b-f
        b-e
        b-c
        b-d
        c-e
        c-d
        c-f
        d-e
        d-f
        e-f");

        let result = find_connected_nodes(&input);

        assert_eq!(result, vec![
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
            String::from("f"),
        ]);
    }

     #[test]
    fn test_part2() {
        let input = sample_graph();
        let result = part2(&input);

        assert_eq!(result, String::from("co,de,ka,ta"));
    }
}

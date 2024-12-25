#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashSet, HashMap};


#[derive(Debug, PartialEq, Eq, Clone)]
enum Operand {
    OR,
    AND,
    XOR,
}

use Operand::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Operation {
    op: Operand,
    in_a: String,
    in_b: String,
    result: String,
}

impl Operation {
    fn new(op: Operand, in_a: String, in_b: String, result: String) -> Self {
        Self { op, in_a, in_b, result }
    }
    fn or(in_a: String, in_b: String, result: String) -> Self {
        Self { op: OR, in_a, in_b, result }
    }
    fn and(in_a: String, in_b: String, result: String) -> Self {
        Self { op: AND, in_a, in_b, result }
    }
    fn xor(in_a: String, in_b: String, result: String) -> Self {
        Self { op: XOR, in_a, in_b, result }
    }

    fn eval(self: &Self, signals: &mut Signals) -> bool {
        if let Some(a) = signals.get(&self.in_a) {
            if let Some(b) = signals.get(&self.in_b) {

                let value = match self.op {
                    AND => a & b,
                    OR => a | b,
                    XOR => a ^ b,
                };

                signals.insert(self.result.clone(), value);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

type Signals = HashMap<String, u8>;
type Operations = HashMap<String, Vec<Operation>>;
type OperationsResult = HashMap<String, Operation>;
type Input = (Signals, Operations);

#[aoc_generator(day24)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let mut signals = Signals::new();
    let mut operands = Operations::new();

    while let Some(l) = lines.next() {
        let line = l.trim();
        if line.is_empty() {
            break;
        }

        let s: Vec<_> = line.split(": ").collect();
        let sname: String = String::from(s[0]);
        let val: u8 = s[1].parse().unwrap();
        signals.insert(sname, val);
    }

    while let Some(l) = lines.next() {
        let line = l.trim();

        let s: Vec<_> = line.split(" -> ").collect();

        let result = String::from(s[1]);

        let s: Vec<_> = s[0].split(" ").collect();

        let op = match s[1] {
            "AND" => AND,
            "XOR" => XOR,
            "OR"  => OR,
            _     => panic!()
        };
        
        let in_a = String::from(s[0]);
        let in_b = String::from(s[2]);


        let operation = Operation::new(op, in_a.clone(), in_b.clone(), result);
        if let Some(ops) = operands.get_mut(&in_a) {
            ops.push(operation.clone());
        } else {
            operands.insert(in_a, vec![operation.clone()]);
        }

        if let Some(ops) = operands.get_mut(&in_b) {
            ops.push(operation.clone());
        } else {
            operands.insert(in_b, vec![operation.clone()]);
        }
    }
    
    (signals, operands)
}

fn eval_circuit(signals: &mut Signals, operations: &Operations) {
    let s2 = signals.clone();
    let mut to_eval = Vec::from_iter(s2.keys());
    let mut evaluated = HashSet::<String>::from_iter(signals.keys().map(|s| s.clone()));

    while let Some(s) = to_eval.pop() {
        if let Some(ops) = operations.get(s) {
            ops.iter().for_each(|op| {
                if !evaluated.contains(&op.result) && op.eval(signals) {
                    evaluated.insert(op.result.clone());
                    to_eval.push(&op.result);
                }
            });
        }
    }
}

#[aoc(day24, part1)]
fn part1((signals, operations): &Input) -> u64 {
    let mut signals = signals.clone();
    eval_circuit(&mut signals, &operations);


    let mut zsig: Vec<_> = signals
        .keys()
        .filter(|k| k.starts_with("z"))
        .collect();

    zsig.sort();

    let mut result: u64 = 0;
    let mut exp = 0;

    for s in zsig {
        let v = signals.get(s).unwrap();
        result += 2u64.pow(exp) * *v as u64;
        exp += 1;
    }
    result
}

fn reverse_operations(operations: &Operations) -> OperationsResult {
    let mut result = OperationsResult::new();

    for (k, v) in operations {
        for op in v {
            result.insert(op.result.clone(), op.clone());
        }
    }

    result
}

fn dependencies(signal: String, signals: &Signals, rev_opers: &OperationsResult) -> HashSet<String> {
    let mut result = HashSet::<String>::new();
    let mut pending = Vec::<&String>::new();
    pending.push(&signal);

    while let Some(s) = pending.pop() {
        if let Some(op) = rev_opers.get(s) {
            println!("+ {s} => {}, {}", &op.in_a, &op.in_b);
            result.insert(op.in_a.clone());
            result.insert(op.in_b.clone());
            pending.push(&op.in_a);
            pending.push(&op.in_b);
        }
    }
    result
}

#[aoc(day24, part2)]
fn part2(input: &Input) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input_1() -> Input {
        parse(
            "x00: 1
             x01: 1
             x02: 1
             y00: 0
             y01: 1
             y02: 0
             
             x00 AND y00 -> z00
             x01 XOR y01 -> z01
             x02 OR y02 -> z02
             x00 AND x01 -> tmp"
        )
    }

    fn sample_input_2() -> Input {
        parse(
            "x00: 1
             x01: 0
             x02: 1
             x03: 1
             x04: 0
             y00: 1
             y01: 1
             y02: 1
             y03: 1
             y04: 1
             
             ntg XOR fgs -> mjb
             y02 OR x01 -> tnw
             kwq OR kpj -> z05
             x00 OR x03 -> fst
             tgd XOR rvg -> z01
             vdt OR tnw -> bfw
             bfw AND frj -> z10
             ffh OR nrd -> bqk
             y00 AND y03 -> djm
             y03 OR y00 -> psh
             bqk OR frj -> z08
             tnw OR fst -> frj
             gnj AND tgd -> z11
             bfw XOR mjb -> z00
             x03 OR x00 -> vdt
             gnj AND wpb -> z02
             x04 AND y00 -> kjc
             djm OR pbm -> qhw
             nrd AND vdt -> hwm
             kjc AND fst -> rvg
             y04 OR y02 -> fgs
             y01 AND x02 -> pbm
             ntg OR kjc -> kwq
             psh XOR fgs -> tgd
             qhw XOR tgd -> z09
             pbm OR djm -> kpj
             x03 XOR y03 -> ffh
             x00 XOR y04 -> ntg
             bfw OR bqk -> z06
             nrd XOR fgs -> wpb
             frj XOR qhw -> z04
             bqk OR frj -> z07
             y03 OR x01 -> nrd
             hwm AND bqk -> z03
             tgd XOR rvg -> z12
             tnw OR pbm -> gnj"
        )
    }

    #[test]
    fn test_parse() {
        let (signals, operations) = sample_input_1();
        assert_eq!(*signals.get(&String::from("x00")).unwrap(), 1);
        assert_eq!(*signals.get(&String::from("x01")).unwrap(), 1);
        assert_eq!(*signals.get(&String::from("x02")).unwrap(), 1);
        assert_eq!(*signals.get(&String::from("y00")).unwrap(), 0);
        assert_eq!(*signals.get(&String::from("y01")).unwrap(), 1);
        assert_eq!(*signals.get(&String::from("y02")).unwrap(), 0);

        assert_eq!(*operations.get(&String::from("x00")).unwrap(), vec![
            Operation::and(String::from("x00"), String::from("y00"), String::from("z00")),
            Operation::and(String::from("x00"), String::from("x01"), String::from("tmp"))
        ]);
        assert_eq!(*operations.get(&String::from("y00")).unwrap(), vec![Operation::and(String::from("x00"), String::from("y00"), String::from("z00"))]);
        assert_eq!(*operations.get(&String::from("x01")).unwrap(), vec![
            Operation::xor(String::from("x01"), String::from("y01"), String::from("z01")),
            Operation::and(String::from("x00"), String::from("x01"), String::from("tmp")),
        ]);
        assert_eq!(*operations.get(&String::from("y01")).unwrap(), vec![Operation::xor(String::from("x01"), String::from("y01"), String::from("z01"))]);
        assert_eq!(*operations.get(&String::from("x02")).unwrap(), vec![Operation::or(String::from("x02"), String::from("y02"), String::from("z02"))]);
        assert_eq!(*operations.get(&String::from("y02")).unwrap(), vec![Operation::or(String::from("x02"), String::from("y02"), String::from("z02"))]);
    }

    #[test]
    fn test_eval_and() {
        let mut signals = Signals::new();
        let and = Operation::and(String::from("a"), String::from("b"), String::from("c"));
        
        signals.insert(String::from("a"), 0);
        signals.insert(String::from("b"), 0);
        and.eval(&mut signals);
        assert_eq!(*signals.get(&String::from("c")).unwrap(), 0);
        
        signals.insert(String::from("a"), 0);
        signals.insert(String::from("b"), 1);
        and.eval(&mut signals);
        assert_eq!(*signals.get(&String::from("c")).unwrap(), 0);

        signals.insert(String::from("a"), 1);
        signals.insert(String::from("b"), 0);
        and.eval(&mut signals);
        assert_eq!(*signals.get(&String::from("c")).unwrap(), 0);
        
        signals.insert(String::from("a"), 1);
        signals.insert(String::from("b"), 1);
        and.eval(&mut signals);
        assert_eq!(*signals.get(&String::from("c")).unwrap(), 1);
    }

    #[test]
    fn test_eval_circuit() {
        let (mut signals, operations) = sample_input_1();
        eval_circuit(&mut signals, &operations);

        assert_eq!(*signals.get(&String::from("z00")).unwrap(), 0);
        assert_eq!(*signals.get(&String::from("z01")).unwrap(), 0);
        assert_eq!(*signals.get(&String::from("z02")).unwrap(), 1);
    }

    #[test]
    fn test_part_1() {
        let input = sample_input_2();
        let result = part1(&input);
        assert_eq!(result, 2024);
    }


    #[test]
    fn test_reverse_operations() {
        let (mut signals, operations) = sample_input_2();

        let op_results = reverse_operations(&operations);

        assert_eq!(op_results.get(&String::from("z00")).unwrap(), &Operation::xor(String::from("bfw"), String::from("mjb"), String::from("z00")));
        assert_eq!(op_results.get(&String::from("kpj")).unwrap(), &Operation::or(String::from("pbm"), String::from("djm"), String::from("kpj")));
    }

    #[test]
    fn test_deps() {
        let (mut signals, operations) = sample_input_2();

        let op_results = reverse_operations(&operations);
        assert_eq!(dependencies(String::from("z00"), &signals, &op_results), HashSet::from_iter([
            String::from("bfw"), String::from("mjb"),
            String::from("ntg"), String::from("fgs"),
            String::from("y04"), String::from("y02"),
            String::from("x00"), String::from("y04"),
            String::from("vdt"), String::from("tnw"),
            String::from("y02"), String::from("x01"),
            String::from("x03"), String::from("x00"), 
        ]));
    }
}

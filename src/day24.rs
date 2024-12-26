#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

use std::collections::{HashSet, HashMap};

const BITS: usize = 46;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Operand {
    OR,
    AND,
    XOR,
}

use Operand::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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

fn swap(swaps: &HashMap<String, String>, operation: Operation) -> Operation {
    let mut result = operation.clone();
    
    if let Some(x) = swaps.get(&result.result) {
        result.result = x.clone();
    }
    // if let Some(x) = swaps.get(&result.in_a) {
    //     result.in_a = x.clone();
    // }
    // if let Some(x) = swaps.get(&result.in_b) {
    //     result.in_b = x.clone();
    // }
    result
}

fn eval_circuit(signals: &mut Signals, operations: &Operations, swaps: &HashMap<String, String>) {
    let s2 = signals.clone();
    let mut to_eval = Vec::<String>::from_iter(s2.keys().map(|s| s.clone()));
    let mut evaluated = HashSet::<String>::from_iter(signals.keys().map(|s| s.clone()));

    while let Some(s) = to_eval.pop() {
        if let Some(ops) = operations.get(&s) {
            ops.iter().for_each(|op| {
                let op = swap(swaps, op.clone());
                if !evaluated.contains(&op.result) && op.eval(signals) {
                    evaluated.insert(op.result.clone());
                    to_eval.push(op.result);
                }
            });
        }
    }
}

#[aoc(day24, part1)]
fn part1((signals, operations): &Input) -> u64 {
    let mut signals = signals.clone();
    eval_circuit(&mut signals, &operations, &HashMap::new());


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

fn reverse_operations(operations: &Operations, swaps: &HashMap<String, String>) -> OperationsResult {
    let mut result = OperationsResult::new();

    for (k, v) in operations {
        for op in v {
            let op = swap(swaps, op.clone());
            result.insert(op.result.clone(), op);
        }
    }

    result
}

fn dependencies(signal: String, rev_opers: &OperationsResult) -> HashSet<String> {
    let mut result = HashSet::<String>::new();
    let mut pending = Vec::<&String>::new();
    pending.push(&signal);

    while let Some(s) = pending.pop() {
        if let Some(op) = rev_opers.get(s) {
            // println!("+ {s} <- {} {:?} {}", &op.in_a, &op.op, &op.in_b);
            result.insert(op.in_a.clone());
            result.insert(op.in_b.clone());
            pending.push(&op.in_b);
            pending.push(&op.in_a);
        }
    }
    result
}

fn dependencies_with_value(signal: String, signals: &Signals, rev_opers: &OperationsResult) -> HashSet<String> {
    let mut result = HashSet::<String>::new();
    let mut pending = Vec::<&String>::new();
    pending.push(&signal);

    while let Some(s) = pending.pop() {
        if let Some(op) = rev_opers.get(s) {
            println!("+ {s}({}) <- {}({}) {:?} {}({})",
                     signals.get(&op.result).unwrap_or(&0),
                     &op.in_a, signals.get(&op.in_a).unwrap_or(&0),
                     &op.op,
                     &op.in_b, signals.get(&op.in_b).unwrap_or(&0));
            result.insert(op.in_a.clone());
            result.insert(op.in_b.clone());
            pending.push(&op.in_b);
            pending.push(&op.in_a);
        }
    }
    result
}

fn to_b(num: u64, digits: usize) -> Vec<u8> {
    let mut result = Vec::new();
    // let digits = num.ilog2() + 1;

    let mut cur = num;
    for d in 0 .. digits {
        result.push((cur % 2u64) as u8);
        cur = cur / 2;
    }
    result.reverse();
    result
}

fn from_b(num: Vec<u8>) -> u64 {
    let mut result = 0;
    let digits = num.len();

    for i in 0 .. digits {
        let exp = (digits - i - 1) as u32;
        result += num[i] as u64 * 2u64.pow(exp);
    }
    result
}


// The problem is 45bit input and result 46bit
fn execute(operations: &Operations, a: u64, b: u64, swaps: &HashMap<String, String>) -> (u64, Signals) {
    let x = to_b(a, BITS);
    let y = to_b(b, BITS);

    // println!("X={x:?}, {}", from_b(x.clone()));
    // println!("Y={y:?}, {}", from_b(y.clone()));

    let mut signals = Signals::new();

    for i in 0 .. x.len() {
        let j = x.len() - i - 1;
        signals.insert(format!("x{:02}", i), x[j]);
    }

    for i in 0 .. y.len() {
        let j = y.len() - i - 1;
        signals.insert(format!("y{:02}", i), y[j]);
    }

    eval_circuit(&mut signals, operations, swaps);

    let mut z = Vec::<u8>::with_capacity(46);

    for i in 0 .. BITS {
        let i = BITS - i - 1;
        let key = format!("z{:02}", i);
        let v = *signals.get(&key).unwrap_or(&0);
        z.push(v);
    }

    // println!("Z={z:?}, {}", from_b(z.clone()));

    (from_b(z), signals)
}


// #[aoc(day24, part2)]
fn show_bad_gates((signals, operations): &Input) -> u32 {
    let kk: HashSet<_> = operations
        .iter()
        .flat_map(|(_, ops)| ops)
        .filter(|op| {
            if op.result.starts_with("z") && op.op != XOR {
                return true;
            }
            if !op.result.starts_with("z") &&
                !op.in_a.starts_with("x") &&
                !op.in_b.starts_with("x") &&
                !op.in_a.starts_with("y") &&
                !op.in_b.starts_with("y") &&
                op.op == XOR
            {
                return true;
            }
                
            false
        })
        .collect();

    for k in kk {
        println!("{:?}", k);
    }
    
    0
}

// I've solved this by using this method to display bad results and then
// by hand checking the bas gates and replacing them.
// The swaps map is a relationship of the gates to be replaced.
#[aoc(day24, part2)]
fn check_sums((signals, operations): &Input) -> String {
    let mut swaps = HashMap::<String, String>::new();
    swaps.insert(String::from("z08"), String::from("cdj"));
    swaps.insert(String::from("cdj"), String::from("z08"));
    
    swaps.insert(String::from("z16"), String::from("mrb"));
    swaps.insert(String::from("mrb"), String::from("z16"));
    
    swaps.insert(String::from("z32"), String::from("gfm"));
    swaps.insert(String::from("gfm"), String::from("z32"));
    
    swaps.insert(String::from("qjd"), String::from("dhm"));
    swaps.insert(String::from("dhm"), String::from("qjd"));
    
    let op_results = reverse_operations(&operations, &swaps);

    // let r = 0 ..= 2u64.pow(16);
    // let r = [256];

    let mut x = 1;
    
    loop {
        // let x = x / 100;
        // let y = x % 100;
        // let x = 0;
        x *= 2;
        let y = x;

        if x >= 2u64.pow(BITS as u32) {
            break;
        }
        
        // println!("=== {x} + {y} = {} ===", x + y);
        
        let (result, signals) = execute(&operations, x, y, &swaps);

        let real = to_b(result, BITS);
        let expected = to_b(x + y, BITS);

        for i in 0 .. BITS {
            if real[i] != expected[i] {
                let j = BITS - i - 1;
                println!("DIFF z{j:02} expected: {} but is {}", expected[i], real[i]);
                let deps = dependencies_with_value(format!("z{j:02}"), &signals, &op_results);
                println!("");
            }
        }

        
        if real != expected {
            // println!(">>>>>> {}", &signals.get(&String::from("z32")).unwrap_or(&0));
            // dependencies_with_value(format!("z32"), &signals, &op_results);
            // 
            // println!(">>>>>> {}", &signals.get(&String::from("z31")).unwrap_or(&0));
            // dependencies_with_value(format!("z31"), &signals, &op_results);
            // 
            // println!(">>>>>> {}", &signals.get(&String::from("pqv")).unwrap_or(&0));
            // dependencies_with_value(format!("pqv"), &signals, &op_results);
            // 
            // println!(">>>>>> {}", &signals.get(&String::from("gfm")).unwrap_or(&0));
            // dependencies_with_value(format!("gfm"), &signals, &op_results);

            // println!("{result}, {} ==> {:?}", x + 1, to_b(result ^ (x+y), BITS));
            panic!();
        }

    }

    // Output format: gates on alphabetical order
    let mut gates: Vec<_> = swaps
        .keys()
        .map(|k| k.clone())
        .collect();

    gates.sort();
    gates.join(",")
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
        eval_circuit(&mut signals, &operations, &HashMap::new());

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

        let op_results = reverse_operations(&operations, &HashMap::new());

        assert_eq!(op_results.get(&String::from("z00")).unwrap(), &Operation::xor(String::from("bfw"), String::from("mjb"), String::from("z00")));
        assert_eq!(op_results.get(&String::from("kpj")).unwrap(), &Operation::or(String::from("pbm"), String::from("djm"), String::from("kpj")));
    }

    #[test]
    fn test_deps() {
        let (mut signals, operations) = sample_input_2();

        let op_results = reverse_operations(&operations, &HashMap::new());
        assert_eq!(dependencies(String::from("z00"), &op_results), HashSet::from_iter([
            String::from("bfw"), String::from("mjb"),
            String::from("ntg"), String::from("fgs"),
            String::from("y04"), String::from("y02"),
            String::from("x00"), String::from("y04"),
            String::from("vdt"), String::from("tnw"),
            String::from("y02"), String::from("x01"),
            String::from("x03"), String::from("x00"), 
        ]));
    }

    #[test]
    fn test_binary_format() {
        assert_eq!(to_b(93, 7), vec![1, 0, 1, 1, 1, 0, 1]);
        assert_eq!(to_b(39, 6), vec![1, 0, 0, 1, 1, 1]);
        assert_eq!(to_b(140, 8), vec![1, 0, 0, 0, 1, 1, 0, 0]);

        assert_eq!(from_b(to_b(93, 42)), 93);
        assert_eq!(from_b(to_b(39, 42)), 39);
        assert_eq!(from_b(to_b(140, 42)), 140);
    }

    /*
    #[test]
    fn test_execute_sum() {
        let (_, operations) = sample_input_2();

        let mut swaps = HashMap::<String, String>::new();
        // swaps.insert(String::from("fgs"), String::from("psh"));
        // swaps.insert(String::from("psh"), String::from("fgs"));
        
        let op_results = reverse_operations(&operations, &swaps);

        for i in vec![(0, 2)] {
            println!("=== {x} + {y} = {} ===", x + y);
            
            let (result, signals) = execute(&operations, x, y, &swaps);

            let real = to_b(result, BITS + 1);
            let expected = to_b(x + y, BITS + 1);
            
            for i in 0 .. BITS + 1 {
                if real[i] != expected[i] {
                    let j = BITS + 1 - i - 1;
                    println!("DIFF z{j:02} expected: {} but is {}", expected[i], real[i]);
                    let deps = dependencies_with_value(format!("z{j:02}"), &signals, &op_results);
                    println!("");
                }
            }

            println!(">>>>>> {real:?} {}", &signals.get(&String::from("z00")).unwrap_or(&0));
            let deps = dependencies_with_value(format!("z00"), &signals, &op_results);

        }
        
        
        todo!()
}
    */
}

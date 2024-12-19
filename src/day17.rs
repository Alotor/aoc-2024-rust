#![allow(unused_variables, unused_mut, dead_code, unused_comparisons, unused_imports)]

type Input = (u64, u64, u64, Vec<u32>);

struct Machine {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ins_p: usize,
    halt: bool,
    output: Vec<u32>,
}

const ADV: u32 = 0;
const BXL: u32 = 1;
const BST: u32 = 2;
const JNZ: u32 = 3;
const BXC: u32 = 4;
const OUT: u32 = 5;
const BDV: u32 = 6;
const CDV: u32 = 7;

const COMBO_A: u32 = 4;
const COMBO_B: u32 = 5;
const COMBO_C: u32 = 6;

impl Machine {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64) -> Self {
        let ins_p = 0;
        let halt = false;
        let output = vec![];
        Self {
           reg_a, reg_b, reg_c, ins_p, halt, output
        }
    }

    fn combo(&self, val: u32) -> u64 {
        match val {
            0 ..= 3 => val as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo operator")
        }
    }

    fn adv(&mut self, operand: u32) {
        let num = self.reg_a;
        let den = 2u64.pow(self.combo(operand) as u32);
        let result = num / den;
        
        // println!("ADV({num}, {den}) = {result}");
        
        self.reg_a = result;
        self.ins_p += 2
    }

    fn bxl(&mut self, operand: u32) {
        let a = self.reg_b;
        let b = operand as u64;
        let result = a ^ b;
        // println!("BXL({a}, {b}) = {result}");

        self.reg_b = result;
        self.ins_p += 2;
    }

    fn bst(&mut self, operand: u32) {
        let a = self.combo(operand);
        let result = a % 8;
        // println!("BST({a}) = {result}");

        self.reg_b = result;
        self.ins_p += 2;
    }

    fn jnz(&mut self, operand: u32) {
        if self.reg_a != 0 {
            // println!("JNZ({operand})");
            self.ins_p = operand as usize;
        } else {
            // println!("JNZ({operand}) SKIP!");
            self.ins_p += 2;
        }
    }

    fn bxc(&mut self, operand: u32) {
        let a = self.reg_b;
        let b = self.reg_c;
        let result = a ^ b;
        // println!("BXC({a}, {b}) = {result}");

        self.reg_b = result;
        self.ins_p += 2;
    }

    fn out(&mut self, operand: u32) {
        let a = self.combo(operand);
        let result = a % 8;
        // println!("OUT({a}) =>> {result}");

        self.output.push(result as u32);
        self.ins_p += 2;
    }

    fn bdv(&mut self, operand: u32) {
        let num = self.reg_a;
        let den = 2u32.pow(self.combo(operand) as u32) as u64;
        let result = num / den;
        
        // println!("BDV({num}, {den}) = {result}");
        
        self.reg_b = result;
        self.ins_p += 2
    }

    fn cdv(&mut self, operand: u32) {
        let num = self.reg_a;
        let den = 2u32.pow(self.combo(operand) as u32) as u64;
        let result = num / den;
        
        // println!("CDV({num}, {den}) = {result}");
        
        self.reg_c = result;
        self.ins_p += 2
    }

    fn run_inst(&mut self, inst: u32, operand: u32) {
        match inst {
            ADV => self.adv(operand),
            BXL => self.bxl(operand),
            BST => self.bst(operand),
            JNZ => self.jnz(operand),
            BXC => self.bxc(operand),
            OUT => self.out(operand),
            BDV => self.bdv(operand),
            CDV => self.cdv(operand),
            _   => panic!()
        }
    }
    
    fn run(&mut self, program: Vec<u32>) {
        self.ins_p = 0;
        self.halt = false;
        self.output = vec![];
        while !self.halt {
            let inst = program.get(self.ins_p).unwrap();
            let operand = program.get(self.ins_p + 1).unwrap();

            self.run_inst(*inst, *operand);

            if self.ins_p >= program.len() {
                self.halt = true;
            }
        }
    }
}


#[aoc_generator(day17)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let line_a: Vec<_> = lines.next().unwrap().trim().split(": ").collect();
    let line_b: Vec<_> = lines.next().unwrap().trim().split(": ").collect();
    let line_c: Vec<_> = lines.next().unwrap().trim().split(": ").collect();

    let reg_a = line_a[1].parse().unwrap();
    let reg_b = line_b[1].parse().unwrap();
    let reg_c = line_c[1].parse().unwrap();

    lines.next();

    let line_program: Vec<_> = lines.next().unwrap().trim().split(": ").collect();
    let program = line_program[1].split(",").map(|i| i.parse::<u32>().unwrap()).collect();

    (reg_a, reg_b, reg_c, program)
}

#[aoc(day17, part1)]
fn part1(input: &Input) -> String {

    let input: Input = input.clone();
    let (reg_a, reg_b, reg_c, program) = input;
    
    let mut m = Machine::new(reg_a, reg_b, reg_c);
    m.run(program);

    let out: Vec<_> = m.output.iter().map(|n| n.to_string()).collect();
    out.join(",")
}

#[aoc(day17, part2)]
fn part2(input: &Input) -> u64 {

    let input: Input = input.clone();
    let (reg_a, reg_b, reg_c, program) = input;

    let mut reg_a = 35184372088832;
    loop {
        if reg_a % 100000000 == 0 {
            println!("{reg_a}");
        }

        let mut m = Machine::new(reg_a, reg_b, reg_c);

        let mut last_output: usize = 0;

        while !m.halt {
            if let (Some(inst), Some(operand)) = (program.get(m.ins_p), program.get(m.ins_p + 1)) {
                m.run_inst(*inst, *operand);

                if m.output.len() > last_output {
                    if m.output.len() > program.len() || m.output[last_output] != program[last_output] {
                        if m.output.len() > 9 {
                            println!("FORCE HALT {reg_a} {:?} {program:?}", m.output);
                        }
                        // If the output is at any point different we restart
                        m.halt = true;
                    } else {
                        last_output = m.output.len();
                    }
                }

                if m.ins_p >= program.len() {
                    m.halt = true;
                }
            } else {
                m.halt = true;
            }
        }

        if m.output == program {
            break;
        }

        reg_a += 1;
    }

    reg_a
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let (a, b, c, program) = parse(
            "Register A: 729
             Register B: 100
             Register C: 333
             
             Program: 0,1,5,4,3,0"
        );

        assert_eq!(a, 729);
        assert_eq!(b, 100);
        assert_eq!(c, 333);
        assert_eq!(program, vec![0,1,5,4,3,0]);
    }

    #[test]
    fn test_combo_operand() {
        let m = Machine::new(19, 15, 14);

        assert_eq!(m.combo(0), 0);
        assert_eq!(m.combo(1), 1);
        assert_eq!(m.combo(2), 2);
        assert_eq!(m.combo(3), 3);
        assert_eq!(m.combo(4), 19);
        assert_eq!(m.combo(5), 15);
        assert_eq!(m.combo(6), 14);
    }

    #[test]
    fn test_adv() {
        let mut m = Machine::new(24, 4, 0);

        m.run(vec![ADV, 3]);
        assert_eq!(m.reg_a, 3);
        assert_eq!(m.ins_p, 2);

        m.reg_a = 24;
        m.run(vec![ADV, COMBO_B]);
        assert_eq!(m.reg_a, 1);
        assert_eq!(m.ins_p, 2);
    }

    #[test]
    fn test_bxl() {
        let mut m = Machine::new(0, 10, 0);

        m.run(vec![BXL, 5]);
        assert_eq!(m.reg_b, 15);
        assert_eq!(m.ins_p, 2);

        m.reg_b = 7;
        m.run(vec![BXL, 1]);
        assert_eq!(m.reg_b, 6);
        assert_eq!(m.ins_p, 2);
    }

    #[test]
    fn test_bst() {
        let mut m = Machine::new(12, 0, 0);

        m.run(vec![BST, 3]);
        assert_eq!(m.reg_b, 3);
        assert_eq!(m.ins_p, 2);

        m.run(vec![BST, COMBO_A]);
        assert_eq!(m.reg_b, 4);
        assert_eq!(m.ins_p, 2);
    }

    #[test]
    fn test_jnz() {
        let mut m = Machine::new(12, 0, 0);

        m.run(vec![JNZ, 4]);
        assert_eq!(m.ins_p, 4);
        
        m.reg_a = 0;
        m.run(vec![JNZ, 4]);
        assert_eq!(m.ins_p, 2);
    }

    #[test]
    fn test_bxc() {
        let mut m = Machine::new(0, 10, 5);

        m.run(vec![BXC, 4]);
        assert_eq!(m.reg_b, 15);

        m.reg_b = 7;
        m.reg_c = 1;
        m.run(vec![BXC, 4]);
        assert_eq!(m.reg_b, 6);
    }

    #[test]
    fn test_out() {
        let mut m = Machine::new(12, 7, 5);

        m.run(vec![OUT, COMBO_A]);
        assert_eq!(m.output, vec![4]);

        m.run(vec![OUT, COMBO_B, OUT, COMBO_C]);
        assert_eq!(m.output, vec![7, 5]);
    }

    #[test]
    fn test_cdv() {
        let mut m = Machine::new(24, 0, 4);

        m.run(vec![BDV, 3]);
        assert_eq!(m.reg_b, 3);
        assert_eq!(m.ins_p, 2);

        m.reg_a = 24;
        m.run(vec![BDV, COMBO_C]);
        assert_eq!(m.reg_b, 1);
        assert_eq!(m.ins_p, 2);
    }

    #[test]
    fn test_bdv() {
        let mut m = Machine::new(24, 4, 0);

        m.run(vec![CDV, 3]);
        assert_eq!(m.reg_c, 3);
        assert_eq!(m.ins_p, 2);

        m.reg_a = 24;
        m.run(vec![CDV, COMBO_B]);
        assert_eq!(m.reg_c, 1);
        assert_eq!(m.ins_p, 2);
    }

    #[test]
    fn test_examples() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut m = Machine::new(0, 0, 9);
        m.run(vec![2, 6]);
        assert_eq!(m.reg_b, 1);
        
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut m = Machine::new(10, 0, 0);
        m.run(vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(m.output, vec![0, 1, 2]);
        
        // If register A contains 2024, the program 0,1,5,4,3,0
        //   would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut m = Machine::new(2024, 0, 0);
        m.run(vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(m.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(m.reg_a, 0);
        
        // If register B contains 29, the program 1,7 would set register B to 26.
        let mut m = Machine::new(0, 29, 0);
        m.run(vec![1, 7]);
        assert_eq!(m.reg_b, 26);
        
        // If register B contains 2024 and register C contains 43690, the program 4,0
        //  would set register B to 44354.
        let mut m = Machine::new(0, 2024, 43690);
        m.run(vec![4, 0]);
        assert_eq!(m.reg_b, 44354);
    }

    #[test]
    fn test_part1() {
        let input = parse(
            "Register A: 729
             Register B: 0
             Register C: 0
             
             Program: 0,1,5,4,3,0"
        );
        assert_eq!(part1(&input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let input = parse(
            "Register A: 2024
             Register B: 0
             Register C: 0
             
             Program: 0,3,5,4,3,0"
        );
        assert_eq!(part2(&input), 117440);
    }
}

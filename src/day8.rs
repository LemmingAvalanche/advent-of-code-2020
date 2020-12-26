use std::dbg;

#[derive(Copy,Clone,PartialEq)]
enum Operation {
    Nop,
    Jmp,
    Acc,
}

#[derive(Copy,Clone)]
struct Instruction {
    count: isize,
    operation: Operation,
    argument: isize,
}

pub struct Program {
    acc: Vec<isize>,
    instructions: Vec<Instruction>,
}
impl Program {
    fn new() -> Program {
        // input length (lines)
        Program {
            acc: Vec::with_capacity(319), instructions: Vec::with_capacity(602)
        }
    }
    fn to_program(s: &str) -> Program {
        let mut program = Program::new();
        for l in s.lines() {
            let mut sw = l.split_ascii_whitespace();
            let o = sw.next().unwrap();
            let arg = isize::from_str_radix(sw.next().unwrap(), 10).unwrap();
            let operation = match o {
                "nop" => Operation::Nop,
                "jmp" => Operation::Jmp,
                "acc" => Operation::Acc,
                _ => panic!("invalid operation")
            };
            program.push(operation, arg);
        }
        program
    }
    fn push(&mut self, operation: Operation, argument: isize) -> () {
        self.instructions.push(Instruction {count: 0, operation: operation, argument: argument});
    }
    fn increment(&mut self, idx: usize) {
        let copy = self.instructions[idx];
        std::mem::replace(&mut self.instructions[idx], Instruction {
            count: copy.count + 1, operation: copy.operation, argument: copy.argument
        });
    }
    fn clone_reset(&self) -> Program {
        let mut clone = Self::new();
        for i in 0..self.instructions.len() {
            let copy = self.instructions[i];
            clone.instructions.push(Instruction {
                count: 0, operation: copy.operation, argument: copy.argument
            });
        }
        clone
    }
    fn do_instruction(&mut self, idx: usize) -> usize {
        self.increment(idx);
        let i = self.instructions.get(idx).unwrap();
        match i.operation {
            Operation::Nop => {
                idx + 1
            },
            Operation::Jmp => {
                (idx as isize + i.argument) as usize
            },
            Operation::Acc => {
                self.acc.push(i.argument);
                idx + 1
            }
        }
    }
    fn is_visited(&self, idx: usize) -> bool {
        self.count(idx) > 0
    }
    fn is_self_loop_instruction(&self, idx: usize) -> bool {
        self.instructions[idx].operation == Operation::Jmp
            && self.instructions[idx].argument == 0
    }
    // 8-1
    pub fn find_accum_before_loop(input: &str) -> isize {
        let mut program = Self::to_program(input);
        let mut idx = 0;
        loop {
            if program.is_visited(idx) {
                return program.accumulator()
            }
            idx = program.do_instruction(idx);
        }
    }
    // Returns sum of the accumulator of it terminates
    fn terminates(&self) -> Option<isize> {
        let mut p = self.clone_reset();
        let mut idx = 0;
        loop {
            if p.termination_condition(idx) {
                return Some(p.accumulator());
            }
            if p.is_visited(idx) || p.is_self_loop_instruction(idx) {
                return None;
            }
            idx = p.do_instruction(idx);
        }
    }
    // Find a terminating program and return the accumulator after termination
    pub fn find_accum_after_termination(input: &str) -> isize {
        let mut program = Self::to_program(input);
        let mut idx = 0;
        if let Some(accum) = program.terminates() {
            return accum;
        }
        // index for the nop/jmp switcher
        let mut switch = program.next_noop_or_jmp_idx(0).unwrap();
        program.switch_nop_or_jmp(switch);
        loop {
            if let Some(accum) = program.terminates() {
                return accum;
            } else {
                // First we need to switch back the nop/jmp that
                // didn’t fix the program (didn’t make it terminate)
                program.switch_nop_or_jmp(switch);

                switch = program.next_noop_or_jmp_idx(switch).unwrap();
                program.switch_nop_or_jmp(switch);
            }
        }
    }
    fn count(&self, idx: usize) -> isize {
        self.instructions[idx].count
    }
    fn accumulator(&self) -> isize {
        self.acc.iter().sum()
    }
    fn termination_condition(&self, idx: usize) -> bool {
        idx > (self.instructions.len() - 1)
    }
    fn next_noop_or_jmp_idx(&self, idx: usize) -> Option<usize> {
        let mut i = idx + 1;
        while i < self.instructions.len() {
            match self.instructions[i].operation {
                Operation::Nop | Operation::Jmp => {
                    return Some(i);
                },
                Operation::Acc => {
                    i += 1;
                }
            }
        }
        None
    }
    fn switch_nop_or_jmp(&mut self, idx: usize) {
        match self.instructions[idx].operation {
            Operation::Nop => {
                self.switch_nop_to_jmp(idx);
            },
            Operation::Jmp => {
                self.switch_jmp_to_nop(idx);
            },
            Operation::Acc => {
                panic!("Did not expect `Acc`");
            }
        }
    }
    fn switch_nop_to_jmp(&mut self, idx: usize) {
        let copy = self.instructions[idx];
        std::mem::replace(&mut self.instructions[idx], Instruction {
            count: copy.count, operation: Operation::Jmp, argument: copy.argument
        });
    }
    fn switch_jmp_to_nop(&mut self, idx: usize) {
        let copy = self.instructions[idx];
        std::mem::replace(&mut self.instructions[idx], Instruction {
            count: copy.count, operation: Operation::Nop, argument: copy.argument
        });
    }
}

#[test]
fn do_instruction_jmp_jumps_to_end() {
    let p = "jmp 3
nop 5
nop 10
nop 4";
    let mut program = Program::to_program(p);
    assert_eq!(3, program.do_instruction(0));
}
#[test]
fn do_instruction_nop_goes_to_next() {
    let p = "jmp 3
nop 5
nop 10
nop 4";
    let mut program = Program::to_program(p);
    assert_eq!(2, program.do_instruction(1));
}
#[test]
fn do_instruction_acc_changes_accumulator() {
    let p = "acc 10";
    let mut program = Program::to_program(p);
    program.do_instruction(0);
    assert_eq!(10, program.accumulator());
}
#[test]
fn do_instruction_calling_instruction_twice_sets_count_to_two() {
    let p = "acc 10";
    let mut program = Program::to_program(p);
    program.do_instruction(0);
    program.do_instruction(0);
    assert_eq!(2, program.count(0));
}
#[test]
fn example_correct_accumulator_before_loop() {
    let p = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(5, Program::find_accum_before_loop(p));
}
#[test]
fn modified_example_from_part_two_terminates_with_correct_accumulator() {
    let p = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
    let program = Program::to_program(p);
    assert_eq!(8, program.terminates().unwrap());
}
#[test]
fn example_terminates_with_correct_accumulator() {
    let p = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(8, Program::find_accum_after_termination(p));
}

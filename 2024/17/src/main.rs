const INPUT: &str = include_str!("input.txt");

struct Program {
    a: u64,
    b: u64,
    c: u64,

    instructions: Vec<u8>,
    instruction_pointer: usize,

    output: Vec<u8>,
}

impl Program {
    fn new() -> Self {
        let mut iter = INPUT.lines();
        let a = iter.next().unwrap()[12..].parse::<u64>().unwrap();
        let b = iter.next().unwrap()[12..].parse::<u64>().unwrap();
        let c = iter.next().unwrap()[12..].parse::<u64>().unwrap();

        iter.next();
        let instructions = iter.next().unwrap()[9..]
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        Self {
            a,
            b,
            c,
            instructions,
            instruction_pointer: 0,
            output: vec![],
        }
    }

    fn run(&mut self) -> String {
        while self.instruction_pointer < self.instructions.len() {
            self.run_instruction();
        }
        self.output()
    }

    fn output(&self) -> String {
        self.output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn run_instruction(&mut self) {
        let should_increment = match self.instructions[self.instruction_pointer] {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => unreachable!(),
        };
        if should_increment {
            self.instruction_pointer += 2;
        }
    }

    fn adv(&mut self) -> bool {
        self.a = self.a / 2u64.pow(self.combo_operand() as u32);
        true
    }

    fn bxl(&mut self) -> bool {
        self.b = self.b ^ (self.literal_operand() as u64);
        true
    }

    fn bst(&mut self) -> bool {
        self.b = self.combo_operand() % 8;
        true
    }

    fn jnz(&mut self) -> bool {
        if self.a == 0 {
            true
        } else {
            self.instruction_pointer = self.literal_operand() as usize;
            false
        }
    }

    fn bxc(&mut self) -> bool {
        self.b = self.b ^ self.c;
        true
    }

    fn out(&mut self) -> bool {
        self.output.push((self.combo_operand() % 8) as u8);
        true
    }

    fn bdv(&mut self) -> bool {
        self.b = self.a / 2u64.pow(self.combo_operand() as u32);
        true
    }

    fn cdv(&mut self) -> bool {
        self.c = self.a / 2u64.pow(self.combo_operand() as u32);
        true
    }

    fn literal_operand(&self) -> u8 {
        self.instructions[self.instruction_pointer + 1]
    }

    fn combo_operand(&self) -> u64 {
        let op = self.literal_operand();
        match op {
            0 | 1 | 2 | 3 => op as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Invalid use of combo operand 7"),
            _ => unreachable!(),
        }
    }
}

fn main() {
    // PART 1
    println!("{}", Program::new().run());

    // PART 2
    let expected_output = Program::new()
        .instructions
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let mut i = 109019928700000;
    while i <= 109019930900000 {
        let mut program = Program::new();
        program.a = i;
        let result = program.run();
        if result == expected_output {
            println!("{i} => DONE!");
            break;
        }
        let trail = program
            .output
            .iter()
            .rev()
            .take(9)
            .rev()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(",");
        if trail == "5,0,3,1,6,5,5,3,0" {
            println!("{i}: {result}");
        }

        i += 1;
    }
}

// 109019100000000..109020300000000
//   109019849000000..109019851200000
//   109019928700000..109019930900000
//     109019930331546 DING DING DING we found it!
//   109019932900000..109019937200000
//
// 109156500000000..109157700000000
//
// 136902000000000..136903200000000

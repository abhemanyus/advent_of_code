use std::fmt::Display;

use advent_of_code::load_file;

fn main() {
    let mut crt = CRT::new();
    // println!("{}", crt);
    let data = load_file!("ten");
    let instructions = data.split('\n').map(Instruction::from);
    let mut cpu = CPU::new(instructions);
    // let mut accumulator = 0;
    while cpu.tick().is_some() {
        // println!(
        //     "Register: {}, Cycle: {}, instruction: {:?}",
        //     cpu.register, cpu.cycle, cpu.instruction
        // );
        crt.tick(cpu.register);
        // if (cpu.cycle - 20) % 40 == 0 {
        //     accumulator += cpu.register * cpu.cycle;
        //     println!("Register: {}, Accumulator: {}", cpu.register, accumulator);
        // }
    }
    // println!("Total cycles: {}, accumulator: {}", cpu.cycle, accumulator);
    println!("{}", crt);
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "noop" => Self::Noop,
            addx => Self::Addx(addx.split_at(5).1.parse().unwrap()),
        }
    }
}

impl Instruction {
    fn steps(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

struct CRT {
    pixels: [bool; 40 * 6],
    pos: usize,
}

impl CRT {
    fn new() -> Self {
        Self { pixels: [false; 40 * 6], pos: 0 }
    }

    fn tick(&mut self, register: i32) {
        if register.abs_diff(self.pos as i32 % 40) <= 1 {
            self.pixels[self.pos] = true;
        }
        self.pos += 1;
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                if self.pixels[y * 40 + x] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

struct CPU<T: Iterator<Item = Instruction>> {
    cycle: i32,
    instructions: T,
    instruction: Instruction,
    register: i32,
    steps: i32,
}

impl<T: Iterator<Item = Instruction>> CPU<T> {
    fn new(instructions: T) -> Self {
        CPU {
            cycle: 0,
            instructions,
            instruction: Instruction::Noop,
            register: 1,
            steps: 1,
        }
    }
    fn tick(&mut self) -> Option<()> {
        // println!("Register: {}", self.register);
        if self.steps == self.instruction.steps() {
            match self.instruction {
                Instruction::Noop => (),
                Instruction::Addx(v) => self.register += v,
            }
            self.instruction = self.instructions.next()?;
            self.steps = 0;
        }
        self.steps += 1;
        self.cycle += 1;
        Some(())
    }
}

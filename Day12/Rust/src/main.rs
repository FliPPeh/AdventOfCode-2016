use std::io::{BufRead, stdin};

#[derive(Debug, Copy, Clone)]
enum Register { A, B, C, D }

impl Register {
    fn as_index(self) -> usize {
        self as usize
    }
}

type Word = i32;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Immediate(Word),
    Register(Register)
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Cpy(Operand, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Operand, Word)
}

struct CPU {
    pc: Word,
    registers: [Word; 4],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            pc: 0,
            registers: [0; 4]
        }
    }

    fn run_program(&mut self, prog: &[Instruction]) {
        while (self.pc as usize) < prog.len() {
            let instr = prog[self.pc as usize];

            self.execute(instr);
            self.pc += 1;
        }
    }

    fn execute(&mut self, i: Instruction) {
        use Instruction::*;

        match i {
            //Cpy(Operand::Register(rs), rd)  => self.registers[rd.as_index()] = self.registers[rs.as_index()],
            //Cpy(Operand::Immediate(is), rd) => self.registers[rd.as_index()] = is,
            Cpy(s, rd) => self.registers[rd.as_index()] = match s {
                Operand::Register(rs) => self.registers[rs.as_index()],
                Operand::Immediate(is) => is
            },

            Inc(rd) => self.registers[rd.as_index()] += 1,
            Dec(rd) => self.registers[rd.as_index()] -= 1,

            //Jnz(Operand::Register(rs), off) if self.registers[rs.as_index()] != 0 => self.pc += off - 1,
            //Jnz(Operand::Immediate(is), off) if is != 0                           => self.pc += off - 1,
            Jnz(s, off) => if match s {
                Operand::Register(rs) => self.registers[rs.as_index()],
                Operand::Immediate(is) => is
            } != 0 {
                self.pc += off - 1;
            },
        }
    }
}

fn main() {
    let stdin = stdin();

    macro_rules! parse_register {
        ($str:expr) => {
            match $str.as_str() {
                "a" => Register::A,
                "b" => Register::B,
                "c" => Register::C,
                "d" => Register::D,
                 _  => panic!("invalid register {}", $str)
            }
        }
    }

    macro_rules! parse_operand {
        ($str:expr) => {
            match $str.parse() {
                Ok(n) => Operand::Immediate(n),
                _     => Operand::Register(parse_register!($str))
            }
        }
    }

    let prog = stdin
        .lock()
        .lines()
        .map(std::io::Result::unwrap)
        .map(|l| l.split(" ").map(str::to_string).collect::<Vec<_>>())
        .map(|p| match p[0].as_str() {
            "cpy" => Instruction::Cpy(parse_operand!(p[1]), parse_register!(p[2])),
            "inc" => Instruction::Inc(parse_register!(p[1])),
            "dec" => Instruction::Dec(parse_register!(p[1])),
            "jnz" => Instruction::Jnz(parse_operand!(p[1]), p[2].parse().unwrap()),
            _ => panic!("invalid instruction {}", p[0])
        })
        .collect::<Vec<_>>();

    let mut cpu = CPU::new();
    
    cpu.run_program(&prog);

    for (r, v) in [Register::A, Register::B, Register::C, Register::D].iter().zip(&cpu.registers) {
        println!("{:?}: {:04}", r, v);
    }
}

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

impl Operand {
    fn fetch(&self, cpu: &CPU) -> Word {
        match *self {
            Operand::Immediate(i) => i,
            Operand::Register(r) => cpu.registers[r.as_index()]
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand)
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

    fn run_program(&mut self, prog: &mut [Instruction]) {
        while (self.pc as usize) < prog.len() {
            let instr = prog[self.pc as usize];

            //println!("{:?}", instr);

            self.execute(instr, prog);
            self.pc += 1;
        }
    }

    fn execute(&mut self, i: Instruction, prog: &mut [Instruction]) {
        use Instruction::*;

        match i {
            Cpy(s, r) => if let Operand::Register(rd) = r {
                self.registers[rd.as_index()] = match s {
                    Operand::Register(rs) => self.registers[rs.as_index()],
                    Operand::Immediate(is) => is
                }
            },

            Inc(r) => if let Operand::Register(rd) = r { self.registers[rd.as_index()] += 1 },
            Dec(r) => if let Operand::Register(rd) = r { self.registers[rd.as_index()] -= 1 },

            Jnz(s, r) => if s.fetch(self) != 0 {
                self.pc += r.fetch(self) - 1;
            },

            Tgl(s) => {
                let npc = (self.pc + s.fetch(self)) as usize;

                if npc < prog.len() {
                    prog[npc] = match prog[npc] {
                        Instruction::Inc(t) => Instruction::Dec(t),
                        Instruction::Dec(t) => Instruction::Inc(t),
                        Instruction::Tgl(t) => Instruction::Inc(t),
                        Instruction::Jnz(a, b) => Instruction::Cpy(a, b),
                        Instruction::Cpy(a, b) => Instruction::Jnz(a, b),

                    }
                }
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

    let mut prog = stdin
        .lock()
        .lines()
        .map(std::io::Result::unwrap)
        .map(|l| l.split(" ").map(str::to_string).collect::<Vec<_>>())
        .map(|p| match p[0].as_str() {
            "cpy" => Instruction::Cpy(parse_operand!(p[1]), parse_operand!(p[2])),
            "inc" => Instruction::Inc(parse_operand!(p[1])),
            "dec" => Instruction::Dec(parse_operand!(p[1])),
            "jnz" => Instruction::Jnz(parse_operand!(p[1]), parse_operand!(p[2])),
            "tgl" => Instruction::Tgl(parse_operand!(p[1])),
            _ => panic!("invalid instruction {}", p[0])
        })
        .collect::<Vec<_>>();

    let mut prog_orig = prog.clone();

    /*
     * Part 1
     */
    let mut cpu = CPU::new();
    
    cpu.registers[Register::A.as_index()] = 7;
    cpu.run_program(&mut prog);

    println!("Part 1");
    for (r, v) in [Register::A, Register::B, Register::C, Register::D].iter().zip(&cpu.registers) {
        println!("{:?}: {:04}", r, v);
    }

    println!("");

    /*
     * Part 2
     */
    cpu = CPU::new();

    cpu.registers[Register::A.as_index()] = 12;
    cpu.run_program(&mut prog_orig);

    println!("Part 2");
    for (r, v) in [Register::A, Register::B, Register::C, Register::D].iter().zip(&cpu.registers) {
        println!("{:?}: {:04}", r, v);
    }
}

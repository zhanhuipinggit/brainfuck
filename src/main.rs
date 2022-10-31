mod opcode;
use std::io::{Write, Read};

use opcode::{Opcode, Code};

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self { 
             stack: vec![0;1] 
        }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0;
        let mut sp = 0;
        loop {
            if pc >= code_len {
                break;
            }

            match code.instrs[pc] {
                Opcode::SHR => {
                    sp += 1;
                    if sp == self.stack. len() {
                        self.stack.push(0);
                    }
                },
                Opcode::SHL => {
                    if sp != 0 {
                        sp -= 1;
                    }
                },
                Opcode::ADD => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                },
                Opcode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;

                },
                Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                },
                Opcode::GETCHAR  => {
                    let mut buf: Vec<u8> = vec![0;1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                },
                Opcode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                },
                Opcode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                },
            }
            pc +=1;

        }
        Ok(())
    }


}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args:Vec<String> = std::env::args().collect();
    let data =  std::fs::read(&args[1])?;
    let mut Interpreter = Interpreter::new();
    Interpreter.run(data);
    // println!("data = {:?}",data);
    // let code = Code::from(data)?;
    // println!("{:?}", code.instrs);
    Ok(())
}
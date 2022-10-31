use std::result;

mod opcode;

#[derive(Debug,PartialEq)]

pub enum IR {
    SHR(u32),
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32),
    JNZ(u32),
}

pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<opcode:: Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instrs: Vec<IR> = Vec::new();
        Ok(Code {instrs})
    }
}

fn main() {

}
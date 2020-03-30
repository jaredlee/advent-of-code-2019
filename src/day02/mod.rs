#![allow(unused)]

use std::fs;

enum OpCode
{
    Add {src1: usize, src2: usize, dst: usize},
    Mul {src1: usize, src2: usize, dst: usize},
    Halt,
}

impl OpCode 
{
    fn size(&self) -> i64
    {
        match self 
        {
            OpCode::Add {..} => 4,
            OpCode::Mul {..} => 4,
            OpCode::Halt {..} => 0,
        }
    }
}

struct RAM { buffer: Vec<i64> }
struct CPU { counter: usize }
struct Computer { ram: RAM, cpu: CPU }

impl RAM
{
    fn size() -> usize { 0x100 }

    fn new() -> RAM 
    {
        let mut ram = RAM { buffer: Vec::new() };
        ram.clear();
        ram
    }

    fn read(&self, address: usize) -> i64 { self.buffer[address] }

    fn write(&mut self, address: usize, value: i64) 
    { 
        self.buffer[address] = value; 
    }
    
    fn clear(&mut self) 
    { 
        self.buffer.clear();
        self.buffer.resize(RAM::size(), 0); 
    }
}

impl CPU
{
    fn new() -> CPU { CPU { counter: 0 } }

    fn clear(&mut self) { self.counter = 0; }

    fn halted(&self, ram: &RAM) -> bool
    {
        match self.decode(ram) 
        {
            OpCode::Halt => true,
            _ => false
        }
    }

    fn cycle(&mut self, ram: &mut RAM)
    {
        let opcode = self.decode(ram);
        self.execute(&opcode, ram);
    }

    fn decode(&self, ram: &RAM) -> OpCode
    {
        let value = ram.read(self.counter);

        let opcode = match value
        {
            1 => OpCode::Add { 
                src1: ram.read(self.counter+1) as usize,
                src2: ram.read(self.counter+2) as usize,
                dst:  ram.read(self.counter+3) as usize,
            },

            2 => OpCode::Mul { 
                src1: ram.read(self.counter+1) as usize,
                src2: ram.read(self.counter+2) as usize,
                dst:  ram.read(self.counter+3) as usize,
            },

            99 => OpCode::Halt,

            _ => panic!("Unrecognized opcode value: {}!", value)
        };

        opcode
    }

    fn execute(&mut self, opcode: &OpCode, ram: &mut RAM)
    {
        match opcode
        {
            &OpCode::Add {src1, src2, dst} =>
            {
                let a = ram.read(src1);
                let b = ram.read(src2);
                ram.write(dst, a+b);
            },

            &OpCode::Mul {src1, src2, dst} =>
            {
                let a = ram.read(src1);
                let b = ram.read(src2);
                ram.write(dst, a*b);
            },

            &OpCode::Halt => (),

            _ => panic!("Unrecognized opcode!")
        }

        self.counter += opcode.size() as usize;
    }
}

impl Computer
{
    fn new() -> Computer
    {
        Computer 
        { 
            ram: RAM::new(),
            cpu: CPU::new(),
        } 
    }

    fn clear(&mut self) 
    {
        self.ram.clear();
        self.cpu.clear();
    }

    fn load_file(&mut self, file: &str)
    {
        let text = fs::read_to_string(file)
            .expect(&format!("Unable to read file {}", file));
        
        self.load_text(&text);
    }

    fn load_text(&mut self, text: &str)
    {
        for (address, token) in text.split(",").enumerate()
        {
            let value = token.parse::<i64>().unwrap();
            self.ram.write(address, value);
        }
    }

    fn init(&mut self, values: &[(usize, i64)])
    {
        for (address, value) in values.iter()
        {
            self.ram.write(*address, *value);
        }
    }
    
    fn exec(&mut self)
    {
        let cpu = &mut self.cpu;
        let ram = &mut self.ram;

        while !cpu.halted(ram)
        {
            cpu.cycle(ram);
        }
    }
} 

pub fn solve_part_one(file: &str) -> i64
{
    let mut computer = Computer::new();

    computer.load_file(file);
    computer.init(&[(1, 12), (2, 2)]);
    computer.exec();

    computer.ram.read(0)
}

pub fn solve_part_two(file: &str) -> i64
{
    let mut computer = Computer::new();

    for x in 0..=99 
    {
        for y in 0..=99
        {
            computer.clear();
            computer.load_file(file);
            computer.init(&[(1,x), (2,y)]);
            computer.exec();
            
            if computer.ram.read(0) == 19690720
            {
                return 100 * x + y;
            }
        }
    }

    0
}

#[cfg(test)]
mod examples
{
    use crate::day02::*;

    #[test]
    fn part_one()
    {
        let mut computer = Computer::new();

        let text = "1,0,0,0,99";

        computer.load_text(text);
        computer.exec();

        assert_eq!(computer.ram.buffer[0..5], [2,0,0,0,99]);

        computer.clear();

        let text = "2,3,0,3,99";

        computer.load_text(text);
        computer.exec();

        assert_eq!(computer.ram.buffer[0..5], [2,3,0,6,99]);

        computer.clear();

        let text = "2,4,4,5,99,0";

        computer.load_text(text);
        computer.exec();

        assert_eq!(computer.ram.buffer[0..6], [2,4,4,5,99,9801]);

        computer.clear();

        let text = "1,1,1,4,99,5,6,0,99";

        computer.load_text(text);
        computer.exec();

        assert_eq!(computer.ram.buffer[0..9], [30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn part_two()
    {
        
    }
}
use day10::Config;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::Lines;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let mut cpu = Cpu::build(contents.lines());

    while cpu.instruction_index < cpu.instructions.len() {
        cpu.tick();
    }

    dbg!(&cpu.watcher);
    let x = cpu.watcher.iter().sum::<i32>();
    println!("{}", x);

    Ok(())
}

struct Cpu {
    xreg: i32,
    instructions: Vec<Instruction>,
    cycle_count: i32,
    instruction_cycle_count: i32,
    instruction_index: usize,
    is_instruction_loaded: bool,
    loaded_instruction: Instruction,
    watcher: Vec<i32>,
}
impl Cpu {
    fn build(lines: Lines) -> Cpu {
        let ilst = lines
            .map(|x| Cpu::convert_line(x))
            .collect::<Vec<Instruction>>();
        let i1 = ilst[0].clone();

        Cpu {
            xreg: 1,
            instructions: ilst,
            cycle_count: 0,
            instruction_index: 0,
            instruction_cycle_count: i1.cycle_time(),
            is_instruction_loaded: false,
            loaded_instruction: i1,
            watcher: Vec::new()
        }
    }

    fn convert_line(line: &str) -> Instruction {
        let v = line.split(" ");
        let a = v.collect::<Vec<&str>>();

        let rins = match a[0] {
            "noop" => Instruction::NOOP,
            _ => {
                let rval = a[1].to_string().parse::<i32>().unwrap();
                Instruction::ADDX(rval)
            }
        };
        return rins;
    }

    fn tick(&mut self) {        
        self.cycle_count = self.cycle_count + 1;

        println!(
            "Starting Cycle {} -- X Reg {} -- Instr {:?} -- Instr Cycle {}",
            self.cycle_count, self.xreg, 
            self.loaded_instruction, self.instruction_cycle_count
        );        

        self.instruction_cycle_count = self.instruction_cycle_count - 1;

        if self.instruction_cycle_count == 0 {
            match self.loaded_instruction {
                Instruction::NOOP => {
                    println!("\tNOOP - Nothing to do");
                }
                Instruction::ADDX(val) => {
                    println!("\tADDX {}", val);
                    self.xreg = self.xreg + val;
                }
            }
            self.instruction_index = self.instruction_index + 1;
            self.is_instruction_loaded = false;
        }
        println!("Ending Cycle {} -- X Reg {}", self.cycle_count, self.xreg);

        let tk = [20, 60, 100, 140, 180, 220];
        if tk.contains(&self.cycle_count) {
            println!("Recording on cnt {} val {}", self.cycle_count, self.xreg);
            self.watcher.push(self.cycle_count * self.xreg);
        }

        if self.is_instruction_loaded == false && self.instruction_index < self.instructions.len() {
            println!(
                "Need to Load Instruction: idx {} - inst {:?}",
                self.instruction_index, self.instructions[self.instruction_index]
            );

            self.loaded_instruction = self.instructions[self.instruction_index];
            self.instruction_cycle_count = self.loaded_instruction.cycle_time();            
            self.is_instruction_loaded = true;
        }        
    }
}

impl fmt::Debug for Cpu {
    //fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //    write!(f, "Point [{} {}]", self.x, self.y)
    //}

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
            .field("xreg", &self.xreg)
            .field("cycle_count", &self.cycle_count)
            .finish()
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    ADDX(i32),
    NOOP,
}
impl Instruction {
    fn cycle_time(&self) -> i32 {
        match *self {
            Instruction::ADDX(_val) => 2,
            Instruction::NOOP => 1,            
        }
    }
}

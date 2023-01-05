use std::error::Error;
use std::fs;
use day5::Config;
use std::collections::VecDeque;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    let (stacks, instructions) = load_data(contents);

    process(stacks, instructions);

    Ok(())
}

fn process(mut stacks: Vec<VecDeque<char>>, instructions: Vec<Instruction>) {
    //println!("{:?}", stacks);

    for i in instructions {
        // Pull the specific boxes
        let mut holder: VecDeque<char> = VecDeque::new();

        println!("Instruction {:?}", i);

        for _ in 0..i.count {
            let d = (stacks[(i.start_stack - 1) as usize]).pop_front().unwrap();
            holder.push_front(d);
        }
        println!("Holder: {:?}", holder);

        for _ in 0..holder.len() {
            let d = holder.pop_front().unwrap();
            println!("D is {}", d);

            stacks[(i.end_stack - 1) as usize].push_front(d);

             println!("{:?}", stacks);
        }

    }

    println!("Final State\n{:?}\n", stacks);

    let tops = stacks.iter().map(|x| x[0] ).collect::<Vec<char>>();

    println!("Top Crates: {:?}", tops);

}

fn load_data(s: String) -> (Vec<VecDeque<char>>, Vec<Instruction>) {
    // First we need to find the split point of the string
    let loc = s.find("move").unwrap();
    let (st_state, instruct) = s.split_at(loc);

    let stacks = load_stacks(st_state);
    let instruct = load_instruct(instruct);

    (stacks, instruct)
}

fn load_stacks(st: &str) -> Vec<VecDeque<char>> {
    let mut stacks: Vec< VecDeque<char> > = Vec::new();


    let lines = st.lines();

    // Figure out how many stacks
    let ls = st.lines()
        .map(|x| (x.len()+1) as i32)
        .max().unwrap();

    let ns = ls / 4;
    for _ in 0..ns {
        let a: VecDeque<char> = VecDeque::new();
        stacks.push(a);
    }

    let rng = 1..ns;

    for l in lines {
        let mut it = l.chars();

        let mut c_count = 0;
        loop {

            let c_opt = it.next();
            //println!("I got {:?}", c_opt);
            if c_opt.is_none() { c_count = 0; break; }

            let c = c_opt.unwrap();
            if c == ' ' { c_count = c_count + 1; }
            else if c == '[' {
                //println!("calc");
                // Okay we are here. First how many spaces did we jump?

                //println!("c_count {}", c_count);
                let tn = c_count / 4;
                let n = it.next().unwrap();

                //println!("\tPutting '{}' in {}", n, tn);
                stacks[tn].push_back(n);
                it.next();
                c_count = c_count + 3
            }
            else if c.to_digit(10).is_some() {
                //It's a number break
                break;
            } else {
                panic!("NO idea shouldn't be here");
            }
        }
    }
    stacks
}

fn load_instruct(st: &str) -> Vec<Instruction> {
    let ls = st.lines();
    let mut instructions: Vec<Instruction> = Vec::new();

    for l in ls {
        let mut cl = l.replace("move", "");
        cl = cl.replace("from", "");
        cl = cl.replace("to", "");

        let d = cl.split_whitespace()
            .map( |x| x.parse().unwrap() )
            .collect::<Vec<i32>>();
        instructions.push(
            Instruction { count: d[0], start_stack: d[1], end_stack: d[2] }
        );
    }
    instructions
}

#[derive(Debug)]
struct Instruction {
    count: i32,
    start_stack: i32,
    end_stack: i32
}
/*
#[derive(Debug)]
struct Team {
    e1: Elf,
    e2: Elf
}
impl Team {
    fn build(l: &str) -> Team {
        let (e1, e2) = l.split(",").collect_tuple().unwrap();
        //println!("{} -- {}", e1, e2);

        Team { e1: Elf::build(e1), e2: Elf::build(e2) }
    }

    fn has_full_overlap(&self) -> bool {
        self.e1.fully_contain_other(&self.e2) || self.e2.fully_contain_other(&self.e1)
    }

    fn has_any_overlap(&self) -> bool {
        self.e1.has_any_overlap_with(&self.e2)
    }
}


#[derive(Debug)]
struct Elf {
    low_sec: i32,
    high_sec: i32
}

impl Elf {
    fn build(l1: &str) -> Elf {
        //println!("{:?}", l1);
        let d = l1.split('-').collect::<Vec<_>>();
        //println!("{:?}", d);

        Elf {
            low_sec: d[0].parse().unwrap(),
            high_sec: d[1].parse().unwrap()
        }
    }

    fn fully_contain_other(&self, e: &Elf) -> bool {
        let low_contained = self.low_sec <= e.low_sec;
        let high_contained = self.high_sec >= e.high_sec;

        low_contained && high_contained
    }

    fn has_any_overlap_with(&self, e: &Elf) -> bool {
        println!("My: ({} - {}), Other: ({} - {})", self.low_sec, self.high_sec, e.low_sec, e.high_sec);
        let d1 = self.low_sec <= e.high_sec;
        let d2 = self.high_sec >= e.low_sec;

        d1 && d2
    }
}
*/

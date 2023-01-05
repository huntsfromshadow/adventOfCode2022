use std::error::Error;
use std::fs;
use day6::Config;
use std::collections::HashSet;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    
    // start point
    let mut start_win: usize = 0;
    loop {
        println!("-------");
        let v = extract_chars_and_check(&contents, start_win, 4);
        if v == true {
            println!("Yep we have hit it");
            break;
        } else {            
            start_win = start_win + 1;    
        }
    }

    println!("Final Spot: {}", start_win + 4);

    Ok(())
}

fn extract_chars_and_check(s: &String, start: usize, length: usize) -> bool {
    let ns = s.clone();
    let e = start + length;
    
    let mut hs: HashSet<char> = HashSet::new();
    let c = &ns[start..e];
    c.chars().for_each(|x| { 
        println!("Adding {}", x);
        hs.insert(x.clone());
    });

    if hs.len() < length {
        return false;
    } else {
        return true;
    }
}

/*
fn process(mut stacks: Vec<VecDeque<char>>, instructions: Vec<Instruction>) {
    //println!("{:?}", stacks);

    for i in instructions {
        for _ in 0..i.count {
            let d = (stacks[(i.start_stack - 1) as usize]).pop_front();
            // println!("Got {}", d.unwrap());
            stacks[(i.end_stack - 1) as usize].push_front(d.unwrap());

            // println!("{:?}", stacks);
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
*/
use day7::Config;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    load_drive(contents);

    Ok(())
}

fn load_drive(s: String) {
    let root: Vec<Data> = Vec::new();

    let mut it = s.lines().peekable();
    let mut path: VecDeque<String> = VecDeque::new();

    loop {
        let r = it.next();
        if r.is_none() {
            break;
        }
        let mut d = r.unwrap().to_string().clone();

        if d.contains("$ cd") {
            d = d.replace("$ cd ", "");
            if d == "/" {
                println!("cd root");
                &path.clear();
            } else if d == ".." {
                println!("One step up");
                path.pop_back();
                dbg!(&path);
            } else {
                println!("It's a cd into a dir");
                path.push_back(d.to_string());
                dbg!(&path);
            }
        } else if d.contains("$ ls") {
            // We need to walk through the ls list
            loop {
                if let None = it.peek() {
                    break;
                }
                let v = it.next().unwrap();
                
                if v.contains("$") == false {
                    // It's a file
                    let x = it.next().unwrap();                    
                    dbg!(x);
                } else {
                    println!("Next is command");
                    break;
                }            
            }            
        }

        // Step 1 we need to figure out what kind of command it is
    }
}

fn parse_result(s: String) {}

enum Data {
    File,
    Directory,
}

enum Instruction {
    CD { path: String },
    LS,
}

struct Directory {}

use std::collections::HashSet;
use std::error::Error;
use std::fs;

use day3::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    //println!("Contents {contents}");

    //let mut rucksacks: Vec<Rucksack> = Vec::new();
    let lines = contents.lines();
    //let mut rnd_number = 1;

    let mut total = 0;
    for s in lines {
        let r = Rucksack::build(s).unwrap();
        let dat = r.find_intersection();

        //println!("{:?}\n{:?}\n{:?}\n\n", r.c1, r.c2, " ");


        dat.iter().for_each(|x| {
            total = total + Rucksack::priority_value(*x);
        });


    }

    println!("Final Total: {}", total);

    Ok(())
}

#[derive(Debug)]
struct Rucksack {
    pub c1: Vec<char>,
    pub c2: Vec<char>,
}

impl Rucksack {
    pub fn build(line: &str) -> Result<Rucksack, &'static str> {
        let mut chars: Vec<char> = line.chars().collect();
        let rl = chars.len() / 2;

        Ok(Rucksack {
            c1: chars.drain(0..rl).collect(),
            c2: chars.drain(0..).collect(),
        })
    }

    fn find_intersection(&self) -> HashSet<char> {
        let mut tmp: HashSet<char> = HashSet::new();
        self.c1
            .iter()
            .filter(|x| self.c2.contains(x))
            .for_each(|x| {
                tmp.insert(*x);
            });
        tmp
    }

    fn priority_value(c: char) -> i32 {
        let v = c as u32;

        if v >= 97 && v <= 122 {
            return (v as i32) - 96;
        }
        else if v >= 65 && v <= 90 {
            return (v as i32) - 38;
        }
        else {
            panic!("Invalid char");
        }
    }
}

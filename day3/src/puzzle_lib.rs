use std::collections::HashSet;
use std::error::Error;
use std::fs;

use day3::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    //println!("Contents {contents}");

    let mut elf_groups: Vec<ElfGroup> = Vec::new();
    let lines = contents.lines();
    //let mut rnd_number = 1;

    let mut total = 0;
    let mut it = lines.into_iter();


    loop {
        let v = it.next();

        if v.is_none() {
            break;
        } else {
            // It's SOME
            let l1 = v.unwrap();
            let l2 = it.next().unwrap();
            let l3 = it.next().unwrap();

            println!("L1: {}, L2: {}, L3: {}", l1, l2, l3);
            elf_groups.push(ElfGroup::build(l1, l2, l3));
        }
    }


    for e in elf_groups {
        println!("{} -- {}", e.badge(), e.badge_priority());
        total = total + e.badge_priority();
    }

    println!("Final Total: {}", total);

    Ok(())
}

#[derive(Debug)]
struct ElfGroup {
    elf1: Rucksack,
    elf2: Rucksack,
    elf3: Rucksack
}
impl ElfGroup {
    pub fn build(elf1_line: &str, elf2_line: &str, elf3_line: &str) -> ElfGroup {
        ElfGroup {
            elf1: Rucksack::build(elf1_line).unwrap(),
            elf2: Rucksack::build(elf2_line).unwrap(),
            elf3: Rucksack::build(elf3_line).unwrap() }
    }

    fn badge(&self) -> char {
        let f1 = self.elf1.full_line.clone();
        let f2 = self.elf2.full_line.clone();
        let f3 = self.elf3.full_line.clone();

        let z = f1.into_iter()
            .filter(|x| f2.contains(x) && f3.contains(x))
            .collect::<HashSet<_>>()
            .iter()
            .next()
            .unwrap()
            .clone();
        z
    }

    fn badge_priority(&self) -> i32 {
        Rucksack::priority_value( self.badge() )
    }
}

#[derive(Debug)]
struct Rucksack {
    pub full_line: Vec<char>,
    pub c1: Vec<char>,
    pub c2: Vec<char>,
}

impl Rucksack {
    pub fn build(line: &str) -> Result<Rucksack, &'static str> {
        let mut chars: Vec<char> = line.chars().collect();
        let rl = chars.len() / 2;

        Ok(Rucksack {
            full_line: chars.clone(),
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

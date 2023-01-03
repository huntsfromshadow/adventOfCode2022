use std::error::Error;
use std::fs;

use day1::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    //println!("Contents {contents}");

    let mut elfs: Vec<Elf> = Vec::new();

    let lines = contents.lines();
    let mut idn = 1;
    let mut tmp_elf: Elf = Elf{
        id_number: idn,
        total_calories: 0
    };


    for s in lines {
        if s == "" {
            elfs.push(tmp_elf);
            idn = idn + 1;
            tmp_elf = Elf{
                id_number: idn,
                total_calories: 0
            };
        } else {
            let c = s.parse::<i32>().unwrap();
            tmp_elf.total_calories = tmp_elf.total_calories + c;
        }
    }

    println!("Elfs Parsed. Finding Highest Calorie");

    let mut high_id = 0;
    let mut high_calorie = 0;
    for e in elfs {
        if e.total_calories > high_calorie {
            high_id = e.id_number;
            high_calorie = e.total_calories;
        }
    }

    println!("Highest Calorie Elf #{}, Total Cals {}", high_id, high_calorie);


    Ok(())
}
#[derive(Debug)]
struct Elf {
    id_number: i32,
    total_calories: i32,
}

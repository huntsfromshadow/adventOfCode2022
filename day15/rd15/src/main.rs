use regex::Regex;
use std::fs;

const FILE_INPUT: &str = "short_input.txt";
const ROW: i32 = 10;

#[derive(Debug, Copy, Clone)]
struct Sensor {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}
impl Sensor {
    fn build(sensor_x: i32, sensor_y: i32, beacon_x: i32, beacon_y: i32) -> Sensor {
        Sensor {
            sensor_x,
            sensor_y,
            beacon_x,
            beacon_y,
        }
    }
    fn distance(&self) -> i32 {
        let x = (self.sensor_x - self.beacon_x).abs();
        let y = (self.sensor_y - self.beacon_y).abs();
        x + y
    }

    fn space_count_for_row(self, row: i32) -> i32 {
        // First is the row within our zone?
        
        println!("{:?} -> {}", self, self.distance());

        if (self.sensor_y..(self.sensor_y + self.distance())).contains(&row) ||
            (self.sensor_y..(self.sensor_y - self.distance())).contains(&row) {

                println!("in row");


                let x_offset = (self.sensor_y - row).abs();
                let side_amt = self.distance() - x_offset;
                let mut total_on_row = (side_amt * 2) + 1;

                // Is beacon on the row?
                //if self.sensor_y == row || self.beacon_y == row {
                //    total_on_row = total_on_row - 1;
                //}
            return total_on_row;
        } else {
            return 0;
        }
    }

}

fn main() {
    let v = read_input();
    
    let t = Sensor::build(
        8, 7,
        2, 10
    );

    let mut t:i32 = 0;

    v.iter().for_each(|s|
    {
        t = t + s.space_count_for_row(ROW)
    });

    println!("{}", t);
   
}


fn read_input() -> Vec<Sensor> {
    let contents = fs::read_to_string(FILE_INPUT).expect("Should be able to read file.");
    contents.lines().map(|line| {
        let re = Regex::new(r".*x=(-?\d+), y=(-?\d+):.* x=(-?\d+),.*y=(-?\d+)").unwrap();
        let res = re.captures(line).unwrap();

        let s = Sensor::build(
            res[1].parse::<i32>().unwrap(),
            res[2].parse::<i32>().unwrap(),
            res[3].parse::<i32>().unwrap(),
            res[4].parse::<i32>().unwrap(),
        );
        return s;
    }).collect()
}

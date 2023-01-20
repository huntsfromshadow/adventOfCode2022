use regex::Regex;
use std::fs;
use std::collections::HashMap;

const FILE_INPUT: &str = "input.txt";

#[derive(Debug)]
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
}

#[derive(Debug)]
enum SpaceType {
    Sensor,
    Beacon,
    NotBeacon
}

#[derive(Debug)]
struct GridSpot {
    x: i32, y: i32,
    spacetype: SpaceType
}
impl GridSpot {
    fn build(x: i32, y: i32, spacetype: SpaceType) -> GridSpot {
        GridSpot { x, y, spacetype }
    }
}

fn walk(h: &mut HashMap<(i32,i32), GridSpot>, ox: i32, oy: i32, dist: i32) {
    if dist == 0 {
        // See if spot is already in hashmap
        if h.get(&(ox, oy)).is_none() {
            h.insert( (ox, oy), GridSpot::build(ox, oy, SpaceType::NotBeacon) );
        }
    } else {
        if h.get(&(ox, oy)).is_none() {
            h.insert( (ox, oy), GridSpot::build(ox, oy, SpaceType::NotBeacon) );
        }

        // We need to spawn the next 4 direcs
        // Up
        walk(h, ox, oy + 1, dist - 1);
        // Left
        walk(h, ox - 1, oy, dist - 1);
        // Down
        walk(h, ox, oy - 1, dist - 1);
        // Right
        walk(h, ox + 1, oy, dist - 1);
    }
}

fn main() {
    let v = read_input();
    let mut h: HashMap< (i32, i32), GridSpot> = HashMap::new();

    v.iter().for_each(|s| {
        fill_grid(s, &mut h);    
    });

    let y: i32 = 10;
    let mut fcount: i32 = 0;

    let ks 
        = h.keys().filter(|k| k.1 == y);

    ks.for_each(|k| {
        let s = h.get( &(k.0, k.1) ).unwrap();
        match s.spacetype {
            SpaceType::NotBeacon => fcount = fcount + 1,
            _ => { }
        }
    });

    println!("{}", fcount);
}

fn fill_grid(s: &Sensor, h: &mut HashMap<(i32, i32), GridSpot>) {
    h.insert( (s.sensor_x, s.sensor_y), GridSpot::build(s.sensor_x, s.sensor_y, SpaceType::Sensor));
    h.insert( (s.beacon_x, s.beacon_y), GridSpot::build(s.beacon_x, s.beacon_y, SpaceType::Beacon));
    walk(h, s.sensor_x, s.sensor_y, s.distance() );
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

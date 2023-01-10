use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    const FILENAME: &str = "short_input.txt";

    let contents = fs::read_to_string(FILENAME).expect("Should have been able to read the file");

    let map = Map::build(contents);
    map.print_map(None);
    map.print_map(Some(true));

    map.find_paths();
}

#[derive(Debug)]
enum Direc {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
enum Reason {
    CAN_STEP,
    OFF_MAP,
    ELEVATION,
    VISITED_ALREADY,
    END,
}

#[derive(Debug)]
struct MapCell {
    row: usize,
    col: usize,
    elev_code: char,
    elev_value: usize,
    is_start: bool,
    is_end: bool,
}
impl MapCell {
    fn build(row: usize, col: usize, elev_code: char) -> MapCell {
        let mut e_code = elev_code;
        let mut is_start = false;
        let mut is_end = false;

        if elev_code == 'S' {
            e_code = 'a';
            is_start = true;
        } else if elev_code == 'E' {
            e_code = 'z';
            is_end = true;
        }
        let elev_value: usize = elev_convert!(e_code);

        MapCell {
            row,
            col,
            elev_code: e_code,
            elev_value,
            is_start,
            is_end,
        }
    }

    fn can_step(&self, d: Direc, visit_list: &VecDeque<(usize, usize)>, map: &Map) -> Reason {
        //println!("~~In can step {:?}, {}, {}", d, self.row, self.col);
        match d {
            Direc::UP => self.can_step_up(visit_list, map),
            Direc::DOWN => self.can_step_down(visit_list, map),
            Direc::LEFT => self.can_step_left(visit_list, map),
            Direc::RIGHT => self.can_step_right(visit_list, map),
        }
    }

    fn can_step_up(&self, visit_list: &VecDeque<(usize, usize)>, map: &Map) -> Reason {
        //println!("~In can step up");
        // up -> -1, 0
        if self.row == 0 {
            return Reason::OFF_MAP;
        } else {
            let nc = (self.row - 1, self.col);

            return self.common_step(nc, &visit_list, &map);
        }
    }

    fn can_step_down(&self, visit_list: &VecDeque<(usize, usize)>, map: &Map) -> Reason {
        //println!("~In can step down");
        // down -> +1, 0
        if self.row == map.max_row - 1 {
            return Reason::OFF_MAP;
        } else {
            let nc = (self.row + 1, self.col);
            return self.common_step(nc, &visit_list, &map);
        }
    }

    fn can_step_left(&self, visit_list: &VecDeque<(usize, usize)>, map: &Map) -> Reason {
        //println!("~In can step left");
        // left -> 0, -1
        if self.col == 0 {
            return Reason::OFF_MAP;
        } else {
            let nc = (self.row, self.col - 1);

            //println!("~~In can step left {}, {}, {:?}", nc.0, nc.1, visit_list);
            return self.common_step(nc, &visit_list, &map);
        }
    }

    fn can_step_right(&self, visit_list: &VecDeque<(usize, usize)>, map: &Map) -> Reason {
        //println!("~In can step right");
        // right -> 0, +1
        println!("{} {}", self.row, map.max_col - 1);
        if self.row == map.max_col - 1 {
            return Reason::OFF_MAP;
        } else {
            println!("goint right");
            let nc = (self.row, self.col + 1);
            return self.common_step(nc, &visit_list, &map);
        }
    }

    fn common_step(
        &self,
        ncord: (usize, usize),
        visit_list: &VecDeque<(usize, usize)>,
        map: &Map,
    ) -> Reason {
       // println!("~In common step");

        if visit_list.contains(&ncord) {
            return Reason::VISITED_ALREADY;
        } else {
            let tcell = map.map.get(&ncord).unwrap();
            if tcell.elev_value <= self.elev_value {
                if tcell.row == map.end_cord.0 && tcell.col == map.end_cord.1 {
                    println!("Found End at {},{}", tcell.row, tcell.col);
                    return Reason::END;
                } else {
                    return Reason::CAN_STEP;
                }
            } else if tcell.elev_value - 1 == self.elev_value {
                if tcell.row == map.end_cord.0 && tcell.col == map.end_cord.1 {
                    return Reason::END;
                } else {
                    return Reason::CAN_STEP;
                }
            } else {
                return Reason::ELEVATION;
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<(usize, usize), MapCell>,
    start_cord: (usize, usize),
    end_cord: (usize, usize),
    max_row: usize,
    max_col: usize,
}
impl Map {
    fn build(map_string: String) -> Map {
        let (map, start_cord, end_cord, max_row, max_col) = Map::load_map(map_string);

        Map {
            map,
            start_cord,
            end_cord,
            max_row, // 1 based
            max_col, // 1 based
        }
    }

    fn load_map(
        map_string: String,
    ) -> (
        HashMap<(usize, usize), MapCell>,
        (usize, usize),
        (usize, usize),
        usize,
        usize,
    ) {
        let lines = map_string.lines();

        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut retval: HashMap<(usize, usize), MapCell> = HashMap::new();

        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);

        for l in lines {
            col = 0;
            for c in l.chars() {
                let new_cell = MapCell::build(row, col, c);

                if new_cell.is_start {
                    start.0 = new_cell.row;
                    start.1 = new_cell.col;
                }

                if new_cell.is_end {
                    end.0 = new_cell.row;
                    end.1 = new_cell.col;
                }

                retval.insert((new_cell.row, new_cell.col), new_cell);
                col = col + 1;
            }
            row = row + 1;
        }
        (retval, start, end, row, col)
    }

    fn print_map(&self, show_val: Option<bool>) {
        let mode = show_val.unwrap_or(false);

        for r in 0..self.max_row {
            for c in 0..self.max_col {
                let cell = self.map.get(&(r, c)).unwrap();
                if mode == false {
                    let ch = cell.elev_code;

                    if cell.is_start {
                        print!("({})", ch);
                    } else if cell.is_end {
                        print!("[{}]", ch);
                    } else {
                        print!(" {} ", ch);
                    }
                } else {
                    let v = self.map.get(&(r, c)).unwrap().elev_value;
                    if v > 10 {
                        if cell.is_start {
                            print!("({})", v);
                        } else if cell.is_end {
                            print!("[{}]", v);
                        } else {
                            print!(" {} ", v)
                        }
                    } else {
                        if cell.is_start {
                            print!("({})", v);
                        } else if cell.is_end {
                            print!("[{}]", v);
                        } else {
                            print!(" {} ", v)
                        }
                    }
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    fn find_paths(&self) {
        let (s_r, s_c) = self.start_cord;
        let mut vec_visit: VecDeque<(usize, usize)> = VecDeque::new();

        self.take_step(s_r, s_c, &mut vec_visit);
    }

    fn take_step(&self, row: usize, col: usize, visit_path: &mut VecDeque<(usize, usize)>) -> bool {
        // get current cell
        let cur_cell = self.map.get(&(row, col)).unwrap();

        // Check up
        let ur = cur_cell.can_step(Direc::UP, &visit_path, &self);
        match ur {
            Reason::END => {
                return true;
            }
            Reason::CAN_STEP => {
                // Up is -1, 0
                visit_path.push_back((row, col));
                println!("Stepping Up from {}, {}. Visit List {:?}", row, col, visit_path);
                let r = self.take_step(row - 1, col, visit_path);
                if !r {
                    //Remove the last step before we try other paths
                    visit_path.pop_back();
                }
            }
            _ => {
                //We can't step print why
                //println!("Can't step from {}, {} UP -> {:?}", row, col, ur)
            }
        }

        // Check left
        let lr = cur_cell.can_step(Direc::LEFT, &visit_path, &self);
        match lr {
            Reason::END => {
                return true;
            }
            Reason::CAN_STEP => {
                // left is 0, -1
                visit_path.push_back((row, col));
                println!("Stepping Left from {}, {}. Visit List {:?}", row, col, visit_path);
                let r = self.take_step(row, col - 1, visit_path);
                if !r {
                    //Remove the last step before we try other paths
                    visit_path.pop_back();
                }
            }
            _ => {
                //We can't step print why
                //println!("Can't step from {}, {} LEFT -> {:?}", row, col, ur)
            }
        }

        // Check down
        let dr = cur_cell.can_step(Direc::DOWN, &visit_path, &self);
        match dr {
            Reason::END => {
                return true;
            }
            Reason::CAN_STEP => {
                // Up is +1, 0
                visit_path.push_back((row, col));
                println!("Stepping down from {}, {}. Visit List {:?}", row, col, visit_path);
                let r = self.take_step(row + 1, col, visit_path);
                if !r {
                    //Remove the last step before we try other paths
                    visit_path.pop_back();
                }
            }
            _ => {
                //We can't step print why
                //println!("Can't step from {}, {} DOWN -> {:?}", row, col, ur)
            }
        }

        println!("!right? {}, {}", row, col);
        // Check right
        let rr = cur_cell.can_step(Direc::RIGHT, &visit_path, &self);
        match ur {
            Reason::END => {
                return true;
            }
            Reason::CAN_STEP => {
                // Right is 0, +1
                visit_path.push_back((row, col));
                println!("Stepping right from {}, {}. Visit List {:?}", row, col, visit_path);
                let r = self.take_step(row, col + 1, visit_path);
                if !r {
                    //Remove the last step before we try other paths
                    visit_path.pop_back();
                }
            }
            _ => {
                //We can't step print why
                println!("Can't step from {}, {} RIGHT -> {:?}", row, col, ur)
            }
        }

        // If we made it here and we haven't found the end then we know it's a dead path
        // return false so the recursion will try another route
        println!("Ret false");
        false
    }
}

#[macro_export]
macro_rules! elev_convert {
    ($a:expr) => {
        // a -> 97
        // z -> 122
        ($a as usize) - 96
    };
}

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    const FILENAME: &str = "input.txt";

    let contents = fs::read_to_string(FILENAME).expect("Should have been able to read the file");

    let mut map = Map::build(contents);
    //map.print_map(None);
    //map.print_map(Some(true));

    map.find_paths();

    map.visit_lengths.sort();
    println!("{:?}", map.visit_lengths);
}

#[derive(Debug)]
enum Direc {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Reason {
    CanStep,
    OffMap,
    Elevation,
    VisitedAlready,
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
}

#[derive(Debug)]
struct Map {
    map: HashMap<(usize, usize), MapCell>,
    start_cord: (usize, usize),
    end_cord: (usize, usize),
    max_row: usize,
    max_col: usize,
    visit_lengths: Vec<usize>,
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
            visit_lengths: Vec::new(),
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

    fn find_paths(&mut self) {
        let (s_r, s_c) = self.start_cord;
        let mut vec_visit: VecDeque<(usize, usize)> = VecDeque::new();

        //println!("Starting Core Find Paths {} {} {:?}", s_r, s_c, vec_visit);
        self.take_step(s_r, s_c, &mut vec_visit);
    }

    fn take_step(&mut self, row: usize, col: usize, visit_path: &mut VecDeque<(usize, usize)>) {
        //println!("\tIn Take Step {} {} {:?}", row, col, visit_path);
        //println!("{}, {}", row, col);
        // get current cell
        //println!("\t\tUp Check");

        visit_path.push_back((row, col));

        // Check up
        let ur = self.can_step(Direc::Up, row, col, &visit_path);
        //println!("\t\tUp Check Result -- {:?}", ur);
        match ur {
            Reason::END => {
                //println!("\t\tGot End in Up Check Step Cnt: {}", visit_path.len());
                println!("Found Path {}", visit_path.len());
                self.visit_lengths.push(visit_path.len());
            }
            Reason::CanStep => {
                // Up is -1, 0
                /*println!(
                    "\t\tStepping Up from {}, {}. Visit List {:?}",
                    row, col, visit_path
                );*/
                self.take_step(row - 1, col, visit_path);
            }
            _ => {}
        }

        // Check left
        //println!("\t\tLeft Check");
        let lr = self.can_step(Direc::Left, row, col, &visit_path);
        //println!("\t\tLeft Check Result -- {:?}", lr);
        match lr {
            Reason::END => {
                //println!("\t\tGot End in Left Check Step Cnt: {}", visit_path.len());
                self.visit_lengths.push(visit_path.len());
            }
            Reason::CanStep => {
                // left is 0, -1
                /*println!(
                    "\t\tStepping Left from {}, {}. Visit List {:?}",
                    row, col, visit_path
                );*/
                self.take_step(row, col - 1, visit_path);
            }
            _ => {}
        }

        // Check down
        //println!("\t\tDown Check");
        let dr = self.can_step(Direc::Down, row, col, &visit_path);
        //println!("\t\tDown Check Result -- {:?}", dr);
        match dr {
            Reason::END => {
                //println!("\t\tGot End in Down Check Step Cnt: {}", visit_path.len());
                self.visit_lengths.push(visit_path.len());
            }
            Reason::CanStep => {
                // Up is +1, 0
                /*println!(
                    "\t\tStepping Down from {}, {}. Visit List {:?}",
                    row, col, visit_path
                );*/
                self.take_step(row + 1, col, visit_path);
            }
            _ => {}
        }

        //println!("\t\tRight Check");
        // Check right
        let rr = self.can_step(Direc::Right, row, col, &visit_path);
        //println!("\t\tRight Check Result -- {:?}", rr);
        match rr {
            Reason::END => {
                //println!("\t\tGot End in Right Check Step Cnt: {}", visit_path.len());
                self.visit_lengths.push(visit_path.len());
            }
            Reason::CanStep => {
                // Right is 0, +1
                /*println!(
                    "\t\tStepping Right from {}, {}. Visit List {:?}",
                    row, col, visit_path
                );*/
                self.take_step(row, col + 1, visit_path);
            }
            _ => {}
        }


        visit_path.pop_back();
    }

    fn can_step(&self, d: Direc, start_r: usize, start_c: usize, visit_list: &VecDeque<(usize, usize)>) -> Reason {
        match d {
            Direc::Up => self.can_step_up(start_r, start_c, visit_list),
            Direc::Down => self.can_step_down(start_r, start_c, visit_list),
            Direc::Left => self.can_step_left(start_r, start_c, visit_list),
            Direc::Right => self.can_step_right(start_r, start_c, visit_list)
        }
    }

    fn can_step_up(&self, start_r: usize, start_c: usize, visit_list: &VecDeque<(usize, usize)>) -> Reason {
        // up -> -1, 0
        if start_r == 0 {
            return Reason::OffMap;
        } else {
            let sc = (start_r, start_c);
            let nc = (start_r - 1, start_c);

            return self.common_step(sc, nc, &visit_list);
        }
    }

    fn can_step_down(&self, start_r: usize, start_c: usize, visit_list: &VecDeque<(usize, usize)>) -> Reason {
        // down -> +1, 0
        if start_r == self.max_row - 1 {
            return Reason::OffMap;
        } else {
            let sc = (start_r, start_c);
            let tc = (start_r + 1, start_c);
            return self.common_step(sc, tc, &visit_list);
        }
    }

    fn can_step_left(&self, start_r: usize, start_c: usize, visit_list: &VecDeque<(usize, usize)>) -> Reason {
        // left -> 0, -1
        if start_c == 0 {
            return Reason::OffMap;
        } else {
            let sc = (start_r, start_c);
            let tc = (start_r, start_c - 1);
            return self.common_step(sc, tc, &visit_list);
        }
    }

    fn can_step_right(&self, start_r: usize, start_c: usize, visit_list: &VecDeque<(usize, usize)>) -> Reason {
        // right -> 0, +1
        if start_c == self.max_col - 1 {
            return Reason::OffMap;
        } else {
            let sc = (start_r, start_c);
            let tc = (start_r, start_c + 1);
            return self.common_step(sc, tc, &visit_list);
        }
    }

    fn common_step(
        &self,
        scord: (usize, usize),
        tcord: (usize, usize),
        visit_list: &VecDeque<(usize, usize)>,
    ) -> Reason {
        if visit_list.contains(&tcord) {
            return Reason::VisitedAlready;
        } else {
            let scell = self.map.get(&scord).unwrap();
            let tcell = self.map.get(&tcord).unwrap();
            if tcell.elev_value <= scell.elev_value {
                if tcell.row == self.end_cord.0 && tcell.col == self.end_cord.1 {
                    //println!("Found End at {},{}", tcell.row, tcell.col);
                    return Reason::END;
                } else {
                    return Reason::CanStep;
                }
            } else if tcell.elev_value - 1 == scell.elev_value {
                if tcell.row == self.end_cord.0 && tcell.col == self.end_cord.1 {
                    return Reason::END;
                } else {
                    return Reason::CanStep;
                }
            } else {
                return Reason::Elevation;
            }
        }
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

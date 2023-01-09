use std::fs;
use std::collections::HashMap;

fn main() {
    const FILENAME: &str = "short_input.txt";

    let contents = fs::read_to_string(FILENAME)
        .expect("Should have been able to read the file");

    let map = Map::build(contents);
    map.find_paths();
}

#[derive(Debug)]
struct Map {
    map: HashMap<(usize, usize), (char, usize)>,
    start_point: (usize, usize),
    end_point: (usize, usize),
    col_length: usize,
    row_length: usize,
}
impl Map {
    fn build(map_string: String) -> Map {
        let (map,
            start, end,
            row, col) = Map::load_map(map_string);


        //println!("{} {}", row, col);

        Map {
            map: map,
            start_point: start,
            end_point: end,
            col_length: col, // 1 based
            row_length: row // 1 based
        }
    }

    fn load_map(map_string: String) ->
        (HashMap<(usize, usize), (char, usize)>,
        (usize, usize), (usize, usize),
        usize, usize) {
        let lines = map_string.lines();

        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut retval: HashMap<(usize, usize), (char, usize)> = HashMap::new();

        let mut start: (usize, usize) = (0,0);
        let mut end: (usize, usize) = (0,0);

        for l in lines {
            col = 0;
            for c in l.chars() {

                let cn: usize;
                if c == 'S' {
                    // Starting position record and convert
                    cn = elev_convert!('a' as usize);
                    start = (row, col);
                } else if c == 'E' {
                    // Ending position
                    cn = elev_convert!('z' as usize);
                    end = (row, col);
                } else {
                    cn = elev_convert!(c as usize);
                }

                retval.insert(
                    (row, col),
                    (c, cn));
                col = col + 1;
            }
            row = row + 1;
        }
        (retval, start, end, row, col)

    }

    fn find_paths(&self) {
        // Start at start
        let s = self.start_point;
        let mut v: Vec<(usize, usize)> = Vec::new();

        self.take_step(self.start_point.0, self.start_point.1, &mut v);
    }

    fn take_step(&self, row: usize, col: usize, visit_path: &mut Vec<(usize, usize)> ) {
        // Takes the step and check all four directions
        println!("\n\nOrigin point: {},{}", row, col);

        visit_path.push( (row,col));

        // Are we at end?
        if row == self.end_point.0 && col == self.end_point.1 {
            println!("At end");
        }

        // Check Up
        if row == 0 {
            // Is space above us
            println!("On Row Edge - No Up Walk");
        } else {
            // Next have already visited that spot?
            if visit_path.contains( &(row - 1, col) ) == false {
                if self.elev_can_walk(row, col, row - 1, col) {
                    return self.take_step(row - 1, col, &visit_path);
                } else {
                    println!("Can't walk up do to elevation")
                }
            }
        }

        // Check Left
        if col == 0  {
            println!("On Col Edge - No Left Walk");
        } else {
            if visit_path.contains( &(row, col - 1) ) == false {
                if self.elev_can_walk(row, col, row, col - 1) {
                    return self.take_step(row, col - 1, visit_path);
                } else {
                    println!("Can't walk down do to elevation");
                }
            }
        }

        // Check Down
        if row == self.row_length - 1 {
            // Is space down us
            println!("On Row Edge - No Down Walk");
        } else {
            if visit_path.contains( &(col, row + 1)) == false {
                if self.elev_can_walk(row, col, row + 1, col) {
                    return self.take_step(row + 1, col, &visit_path);
                } else {
                    println!("Can't walk down do to elevation");
                }
            }
        }

        // Check Right
        if col == self.col_length - 1 {
            println!("On Col Edge - No Right Walk");
        } else {
            if visit_path.contains( &(row, col + 1)) == false {
                if self.elev_can_walk(row, col, row, col + 1) {
                    return self.take_step(row, col + 1, &visit_path);
                } else {
                    println!("Can't walk down do to elevation");
                }
            }
        }

    }

    fn elev_can_walk(&self,
        origin_row: usize, origin_col: usize,
        target_row: usize, target_col: usize) -> bool {

        let origin_elev = self.map.get( &(origin_row, origin_col) ).unwrap().1;
        let target_elev = self.map.get( &(target_row, target_col) ).unwrap().1;

        println!("Comparing Orig:{} to Tar:{}", origin_elev, target_elev);

        if origin_elev > target_elev {
            println!("Can go elev down");
            return true;
        } else if origin_elev == target_elev || origin_elev == target_elev - 1 {
            println!("can go elev up");
            return true;
        } else {
            println!("Can't go");
            return false;
        }
    }

}

#[macro_export]
macro_rules! elev_convert {
    ($a:expr) => {
        // a -> 97
        // z -> 122
        // A -> 65
        // Z -> 90
        if($a > 90) {
            $a - 96
        } else {
            $a - 38
        }
    };
}
use core::fmt;
use std::collections::HashMap;
use std::str::Lines;
use day8::Config;
use std::error::Error;
use std::fs;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let grid = setup_grid(contents.lines());

    dbg!(&grid);

    let cnt = grid.count_visible().clone();
    println!("Visible Trees: {}", cnt );

    Ok(())
}

fn setup_grid(ls: Lines) -> Grid {
    let mut row_cord: usize = 0;
    let mut col_cord: usize = 0;

    let mut ret_grid = Grid::build();

    for l in ls {
        let c_itr = l.chars().into_iter();
        for c in c_itr {
            let height = c.to_digit(10).unwrap() as u32;
            ret_grid.add_tree(row_cord, col_cord, height);
            col_cord = col_cord + 1;
        }
        row_cord = row_cord + 1;
        col_cord = 0;
    }

    ret_grid
}


struct Grid {
    total_rows: usize,
    total_cols: usize,
    grid: HashMap<(usize, usize),u32>
}
impl Grid {
    fn build() -> Grid {
        Grid {
            total_cols: 0,
            total_rows: 0,
            grid: HashMap::new()
        }
    }

    fn add_tree(&mut self, row: usize, col: usize, height: u32) {
       self.grid.insert( (row, col), height);

       if row + 1 > self.total_rows {
        self.total_rows = row + 1;
       }

       if col + 1 > self.total_cols {
        self.total_cols = col + 1;
       }
    }

    fn get_height(&self, row: usize, col:usize) -> u32 {
        *self.grid.get(&(row, col)).unwrap()
    }

    fn is_tree_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 { true }
        else if col == 0 { true }
        else if col == (self.total_cols - 1) { true }
        else if row == (self.total_rows - 1) { true }
        else {
            let height = self.get_height(row,col);
            let ur = self.check_visible_up(row, col, height);
            let lr = self.check_visible_left(row, col, height);
            let dr = self.check_visible_down(row, col, height);
            let rr = self.check_visible_right(row, col, height);

            ur || lr || dr || rr

        }
    }

    fn count_visible(&self) -> u32 {
        let mut total: u32 = 0;
        for r in 0..(self.total_rows) {
            for c in 0..(self.total_cols) {
                if self.is_tree_visible(r, c) {
                    total = total + 1;
                }
            }
        }
        total
    }

    fn check_visible_up(&self, row: usize, col: usize, height: u32) -> bool {
        let l_row = row - 1;
        let l_col = col;

        let uh = self.get_height(l_row, l_col);
        let res = uh < height;
        if l_row == 0 {
            return res;
        } else {
            self.check_visible_up(l_row, l_col, height) && res
        }
    }

    fn check_visible_left(&self, row: usize, col:usize, height: u32) -> bool {
        let l_row = row;
        let l_col = col - 1;

        let uh = self.get_height(l_row, l_col);
        let res = uh < height;
        if l_col == 0 {
            return res;
        } else {
            self.check_visible_left(l_row, l_col, height) && res
        }
    }

    fn check_visible_down(&self, row: usize, col:usize, height: u32) -> bool {
        let l_row = row + 1;
        let l_col = col;

        let uh = self.get_height(l_row, l_col);
        let res = uh < height;
        if l_row == (self.total_rows - 1) {
            return res;
        } else {
            self.check_visible_down(l_row, l_col, height) && res
        }
    }

    fn check_visible_right(&self, row: usize, col:usize, height: u32) -> bool {
        let l_row = row;
        let l_col = col + 1;

        let uh = self.get_height(l_row, l_col);
        let res = uh < height;

        if l_col == (self.total_cols - 1) {
            return res;
        } else {
            self.check_visible_right(l_row, l_col, height) && res
        }
    }


}
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_str = String::from("\n");

        for r in 0..(self.total_rows) {
            let mut row_num_str = String::from("");
            let mut row_vis = String::from("");

            for c in 0..(self.total_cols) {
                let height = self.grid.get( &(r, c) ).unwrap();
                let c_str = height.to_string();
                row_num_str.push_str(&c_str);

                let vis = self.is_tree_visible(r, c);
                if vis { row_vis.push_str("v"); }
                else { row_vis.push_str("i"); }
            }
            out_str.push_str(&row_num_str);
            out_str.push_str("     ");
            out_str.push_str(&row_vis);
            out_str.push_str("\n");
        }
        write!(f, "{}", out_str)
    }
}


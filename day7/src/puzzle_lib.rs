use core::fmt::Debug;
use day7::Config;
use itertools::Itertools;
use regex::Regex; // extern crate regex;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    load_drive(contents);

    Ok(())
}

#[derive(Debug)]
struct Data {
    name: String,
    is_direc: bool,
    size: i32,
    contains: Vec<Data>,
}
impl Data {
    fn build_direc(name: String) -> Data {
        Data {
            name,
            is_direc: true,
            size: 0,
            contains: Vec::new(),
        }
    }

    fn build_file(name: String, size: i32) -> Data {
        Data {
            name,
            is_direc: false,
            size,
            contains: Vec::new(),
        }
    }

    fn add_data(&mut self, data: Data) {
        self.contains.push(data);
    }
}

#[derive(Debug)]
struct Drive {
    pwd: VecDeque<String>,
    root: Data,
}
impl Drive {
    fn build() -> Drive {
        Drive {
            root: Data {
                name: String::from("/"),
                is_direc: true,
                size: 0,
                contains: Vec::new(),
            },
            pwd: VecDeque::from(vec![String::from("/")]),
        }
    }

    fn change_directory(&mut self, dname: String) {
        if dname == "/" {
            self.pwd.clear();
            self.pwd.push_front(String::from("/"));
        } else if dname == ".." {
            _ = self.pwd.pop_back()
        } else {
            self.pwd.push_back(dname);
        }
    }

    fn print_current_pwd(&self) {
        println!("\tCurrent PWD: {:?}", self.pwd);
    }

    fn get_current_directory(&mut self) -> &mut Data {
        if self.pwd.len() == 1 {
            return &mut self.root;
        } else {
            let mut lpwd = self.pwd.clone();

            let mut d: &Data = &self.root;
            for _ in 0..lpwd.len() {
                let n = lpwd.pop_front();
                if n.is_none() {
                    break;
                }

                let nn = n.unwrap();
                if nn == "/" {
                    continue;
                } else {
                    let cdat = &mut d.contains.iter();
                    d = &mut cdat.find(|x| {
                            return x.is_direc && x.name == nn;
                        })
                        .unwrap();
                }
            }
            let mut retval = d;
            return &mut retval;
        }
    }

    fn add_directory(&mut self, dname: String) {
        let direc = self.get_current_directory();
        direc.add_data(Data::build_direc(dname));
    }

    fn add_file(&mut self, fname: String, fsize: i32) {
        let direc = self.get_current_directory();
        direc.add_data(Data::build_file(fname, fsize));
    }
}

fn load_drive(s: String) {
    let mut drive = Drive::build();

    dbg!(&drive);

    // Our mutator set as peekable so we can look ahead
    let mut it = s.lines().peekable();

    // Regs
    let re_cd = Regex::new(r"\$ cd (.*)").unwrap();
    let re_ls = Regex::new(r"\$ ls").unwrap();

    // Walk through each line and process it
    loop {
        let box_line = it.next();
        if box_line.is_none() {
            break;
        }
        let line = box_line.unwrap();
        println!("Instr: {}", line);

        /* HANDLING CD */
        if re_cd.is_match(line) == true {
            println!("\tCD Detected");
            let name = line
                .split(" ")
                .collect_tuple::<(&str, &str, &str)>()
                .unwrap()
                .2;
            drive.change_directory(name.to_string());
            drive.print_current_pwd();
        }
        /* END CD */
        /* HANDLING LS  */
        else if re_ls.is_match(line) == true {
            println!("\tLS Detected");
            drive.print_current_pwd();

            loop {
                let peek_box_line = it.peek();
                if peek_box_line.is_none() || peek_box_line.unwrap().starts_with("$") {
                    println!("\t\tNext is instruct or end of file");
                    break;
                } else {
                    let ls_line = it.next().unwrap();
                    println!("\t\tLS Line {}", ls_line);
                    let data = ls_line.split(" ").collect_tuple::<(&str, &str)>().unwrap();
                    if data.0 == "dir" {
                        drive.add_directory(String::from(data.1));
                    } else {
                        drive.add_file(String::from(data.1), data.0.parse().unwrap());
                        dbg!(&drive);
                    }
                }
            }
        }

        //                 let ls_line = it.next().unwrap();
        //                 println!("\t\tLS Line {}", ls_line);
        //                 let data = ls_line.split(" ").collect_tuple::<(&str, &str)>().unwrap();

        //                 if (data.0 == "dir") {
        //                     let new_dir = Data {
        //                         contains: Vec::new(),
        //                         local_name: data.1.to_string(),
        //                         is_direc: true,
        //                         size: 0,
        //                     };
        //                     println!("Doing push");
        //                     direc.contains.push(new_dir);
        //                 } else {
        //                     let new_file: Data = Data {
        //                         contains: Vec::new(),
        //                         local_name: data.1.to_string(),
        //                         is_direc: false,
        //                         size: data.0.parse::<i32>().unwrap(),
        //                     };
        //                     direc.contains.push(new_file);
        //                 }
        //             }
        //         }
    }
}

//}
//}

/*fn get_direc(pwd: &VecDeque<String>, root: &mut Data  ) -> Data  {
    println!("Get direc");
    let mut pwd_local = pwd.clone();

    // Drop first which is root
    pwd_local.pop_front();

    let dn = pwd_local[0].clone();

    let d = root.contains.into_iter().find(|x| {
        return x.is_direc == true && x.local_name == dn
    }).unwrap();
    dbg!(&d);

    if pwd_local.len() > 1 {
        panic!("Come back");
    }

    d
}*/
/*
         else if re_ls.is_match(&d) {
            // It's a ls
            // Loop through the files till we find a cmd
            loop {
                // Look ahead one
                let nd = it.peek();
                if nd.is_none() || nd.unwrap().starts_with("$") == true {
                    // Break the loop and return up
                    break;
                } else {
                    core.register_file( it.next().unwrap().to_string() )
                }
            }
        }
    } else {
        println!("Skipping");
    }

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




    let mut core = Core::build();


        // Step 1 we need to figure out what kind of command it is
}



#[derive(Debug)]
struct Core {
    drive: HashMap<String, Direc>,
    pwd: Pwd
}
impl Core {
    fn build() -> Core {
        Core {
            drive: HashMap::new(),
            pwd: Pwd::build()
        }
    }

    fn change_pwd(&mut self, path: &str) {
        println!("\tChange PWD -- {}", path);
        match path {
            "/" => self.pwd.clear(),    // Just clear the pwd
            ".." => self.pwd.pop_back_drop(), // Just pop the back path
            _ => self.pwd.push_back( path.clone().to_string() )
        }
        println!("Current PWD - {:?}", self.pwd);
    }

    // Register a file with the drive - we don't have to register direcs as we use the pwd to handle that
    fn register_file(&mut self, ls_line: String) {
        println!("\tRegistering File {}", ls_line);

        // We need to get our current dir entry
        let pid = self.pwd.convert_to_map_id();
        println!("\t\tOur PWD id: ~{}~", pid);

        let direc: &Direc = self.get_directory(&pid);
        println!("\t\tDirec Back: {:?}", direc);

        let data: Data;

        if ls_line.starts_with("dir") {
            let r = ls_line.split(" ").collect_tuple::<(&str, &str)>().unwrap();
            let name = (r.1).to_string().clone();

            // Need to register the directory
            data = Data {
                is_direc: true,
                size: 0,
                name
            };
        } else {
            // Need to
            let r = ls_line.split(" ").collect_tuple::<(&str, &str)>().unwrap();
            let size = (r.0).parse::<i32>().unwrap();
            let name = (r.1).to_string().clone();

            data = Data {
                is_direc: false,
                size,
                name,
            };

            //direc.add_file(data);

        }


        dbg!(&self);
        panic!("hold");
    }

    fn get_directory(&mut self, s: &String) -> &Direc {

        if self.drive.contains_key(s) == false {
            let d = Direc::build();
            self.drive.insert(s.clone().to_string(), d);
        }
        dbg!(self.drive.get(s));

        self.drive.get(s).unwrap()
    }
}

#[derive(Debug)]
struct Pwd {
    parts: VecDeque<String>
}

impl Pwd {
    fn build() -> Pwd {
        Pwd { parts: VecDeque::new() }
    }

    fn convert_to_map_id(&self) -> String {
        return self.parts.iter().join("/")
    }

    fn clear(&mut self) {
        self.parts.clear();
    }

    fn pop_back_drop(&mut self) {
        self.parts.pop_back();
    }

    fn push_back(&mut self, s: String) {
        self.parts.push_back(s);
    }
}

#[derive(Debug)]
struct Direc {
    contains: Vec<Data>
}
impl Direc {
    fn build() -> Direc {
        Direc { contains: Vec::new() }
    }

    fn add_file(&mut self, f: Data) {
        self.contains.push(f);
    }
}



#[derive(Debug)]
struct Data {
    is_direc: bool,
    size: i32,
    name: String,
}
*/

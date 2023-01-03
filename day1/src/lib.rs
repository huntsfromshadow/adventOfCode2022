

pub struct Config {
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 1 {
            return Err("Need to give the input file path/name");
        }
        else if args.len() > 2 {
            return Err("Too Many Arguments");
        }

        let file_path = args[1].clone();

        Ok( Config { file_path } )
    }
}



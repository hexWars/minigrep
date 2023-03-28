use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);


    let contents = fs::read_to_string(config.file_path).expect("Shouold have been able to read the file");

    println!("With text:\n{contents}");

}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new (args: &[String]) -> Result<Config, &'static str> {
        if (args.len() < 3) {
            return Err("not enough argumentrk")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path } )
    }
}


// test mod 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq! (2+2, 4);
    } 
}


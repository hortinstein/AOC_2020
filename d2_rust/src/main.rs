use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
fn check_pw_validity(pw: String ) -> bool {
    let mut split = pw.split(":");
    for item in split{

        println!("{}",item);
    }
    true
    
}

fn main() {
    let mut pw_vec: Vec<String> = vec![];
    let mut count:i32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(pw) = line {
                println!("{}", pw);
                pw_vec.push(pw);
            }
        }
    } else {
        println!("error on input")
    }
    for pw in pw_vec{
        if check_pw_validity(pw){
            count=count+1;
        }
    }
    println!("{}",count)
    
    
}

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
    let rule_pass_split: Vec<&str> = pw.split(":").collect();
    let rule = rule_pass_split[0];
    let range_letter_split: Vec<&str> = rule.split(" ").collect();
    
    let pass = rule_pass_split[1];
    let range = range_letter_split[0];
    let letter = range_letter_split[1];
    
    let upper_lower_split: Vec<&str> = range.split("-").collect();
    let upper = upper_lower_split[1].parse::<usize>().unwrap();
    let lower = upper_lower_split[0].parse::<usize>().unwrap();
    
    let letter_count = pass.matches(letter).count();
    println!("{} {}",pw,letter_count);
    if letter_count <= upper && letter_count >= lower {
        true
    } else {
        false
    }
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

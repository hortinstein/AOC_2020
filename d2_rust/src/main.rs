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
fn check_pw_validity(pw: String ) -> (bool,bool) {
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
    //println!("{} {}",pw,letter_count);
    
    let first_letter = pass.as_bytes()[lower];
    let last_letter = pass.as_bytes()[upper];
    println!("{}",pass);
    let letter_c = letter.as_bytes()[0];

    let bool2 = (first_letter as char == letter_c as char ||
            last_letter as char == letter_c as char) && 
            first_letter as char != last_letter as char;

    println!("{} pass {} target {} first {} last {}; bool {}", range,
                                                   pass,letter_c as char,
                                                   first_letter as char,
                                                   last_letter as char, 
                                                   bool2);
    

    return (letter_count <= upper && letter_count >= lower,bool2)
}
    



fn main() {
    let mut pw_vec: Vec<String> = vec![];
    let mut count1:i32 = 0;
    let mut count2:i32 = 0;
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

    let mut p1:bool;
    let mut p2:bool;

    for pw in pw_vec{
        let (p1,p2) = check_pw_validity(pw);
        if p1{
            count1=count1+1;
        }
        if p2{
            count2=count2+1;
        }
    }
    println!("prob1 {}",count1);
    println!("prob2 {}",count2);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "1-3 a: abcde".to_string();

        assert_eq!((true,true),check_pw_validity(data));
    }
    #[test]
    fn sample_02() {
        let data = "1-3 b: cdefg".to_string();

        assert_eq!((false,false),check_pw_validity(data));
    }
    #[test]
    fn sample_03() {
        let data = "2-9 c: ccccccccc".to_string();

        assert_eq!((true,false),check_pw_validity(data));
    }

}
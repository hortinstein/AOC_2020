use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate colored; // not needed in Rust 2018

use colored::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn remove_whitespace(s: &mut String) {
//     s.retain(|c| !c.is_whitespace());
// }

// fn remove_whitespace_ref(s: &str) -> String {
//     s.chars().filter(|c| !c.is_whitespace()).collect()
// }

// fn print_slope(slope: String,slice_len: usize,i:usize){

// }

fn tree_collision(slope: Vec<String>) -> u32 {
    let mut tree_count:u32 = 0;
    let mut y:usize = 1;
    //hard coded slice len
    println!("{}",slope[y].len());
    let slope_len= slope.len();
    let mut done = false;
    let mut x:usize = 0;
    while !done {
        println!("{}",slope[y]);
        let s: u8 = slope[y].as_bytes()[x % slope_len];
        if '#' as char == s as char{
            tree_count+=1;
        }
        y += 1;
        x += 1;
        if y> slope_len {
            done = true;
        }
    }
    
    tree_count
}

fn main() {

    let mut slope: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(slope_slice) = line {
                slope.push(slope_slice);
            }
        }
    } else {
        println!("error on input")
    }
    tree_collision(slope);


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        let slope_vec: Vec<String> = data.split("\n").map(|s| s.to_string()).collect();
        assert_eq!(7,tree_collision(slope_vec));
    }
 
}
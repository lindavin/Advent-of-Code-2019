use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut fuel_sum = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../puzzle-inputs/day-one.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let i = match ip.parse::<i32>() {
                  Ok(i) => i,
                  Err(_e) => {
                       -1
                    }
                };

                fuel_sum = fuel_sum + i/3 - 2;
            }
        }
        println!("{}", fuel_sum);
    }
}

// Source: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
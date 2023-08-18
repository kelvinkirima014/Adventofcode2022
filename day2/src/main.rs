//use std::fs;

fn main() {

      println!(
        "{}",
        include_bytes!("input.txt")
            .split(|byte| *byte == b'\n')
            .filter(|l| !l.is_empty()) 
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
            .sum::<i16>(),
    );
    
    // let input = fs::read_to_string("src/input.txt").expect("Failed to read from input file");

    // let split_input: Vec<_> = input.split('\n').collect();


    // A - Rock - 1 win - 7
    //B  - Paper - 2 win 8
    //C - Scissors - 3 win 9 


   
}

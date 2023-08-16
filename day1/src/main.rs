use std::fs;


fn main() {

    let input_file = fs::read_to_string("src/input.txt").expect("Failed to read from file");

    let sets: Vec<_> = input_file.split("\n\n").collect();

    let mut top_sums = [i32::MIN; 3];

    for set in sets {

        let current_sum = set.lines().filter_map(|line| line.parse::<i32>().ok()).sum();

        if current_sum > top_sums[0]{

            top_sums[0] = current_sum;

            println!("set with the largest sum is: {:?}", top_sums[0]);

        } else if current_sum > top_sums[1] {
            
            top_sums[1] = current_sum;

            println!("second largest sum is: {:?}", top_sums[1]);

        } else if current_sum > top_sums[2] {
            
            top_sums[2] = current_sum;

            println!("third largest sum is: {:?}", top_sums[2]);
            
        }

    }


}
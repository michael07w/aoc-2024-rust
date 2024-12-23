use std::fs;
use std::io::{prelude::*, BufReader};


fn calibration_result(eq_result: i128, operands: &Vec<i128>, mut temp_result: i128, pos: usize) -> bool {
    if pos == operands.len() {
        if temp_result == eq_result {
            return true
        }
        else {
            return false
        }
    }

    // Try addition
    let next_pos: usize = pos + 1;
    temp_result += operands[pos];
    if calibration_result(eq_result, operands, temp_result, next_pos) {
        return true
    }
    temp_result -= operands[pos];

    // Try multiplication
    temp_result *= operands[pos];
    if calibration_result(eq_result, operands, temp_result, next_pos) {
        return true
    }
    temp_result /= operands[pos];

    // Try concatenation
    let mut val_copy = operands[pos];
    let mut num_digits: u32 = 0;
    while val_copy > 0 {
        val_copy /= 10;
        num_digits += 1;
    }
    temp_result *= 10_i128.pow(num_digits);
    temp_result += operands[pos];
    if calibration_result(eq_result, operands, temp_result, next_pos) {
        return true
    }

    false
}


fn day_seven() {
    let mut result = 0;

    // Read file contents line-by-line
    let file_result = fs::File::open("src/seven-input.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {error:?}"),
    };

    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem reading line: {error:?}"),
        };

        // Extract equation result
        let mut iter = line.char_indices();
        let mut end = 0;
        for (idx, c) in line.chars().enumerate() {
            if c == ':' {
                (end, _) = iter.nth(idx).unwrap();
                break;
            }
        }
        let eq_result_str = &line[0..end];
        let eq_result: i128 = eq_result_str.parse().unwrap();

        // Extract operands
        let operands: Vec<i128> = line[(end + 1)..]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Evaluate validity of the equation
        let pos = 1;
        let temp_result = operands[0];
        if calibration_result(eq_result, &operands, temp_result, pos) {
            result += eq_result;
            
            //print!("{eq_result}: ");
            //println!("{:?}", operands);
        }
    }

    println!("Final Result: {result}.");
}


fn main() {
    day_seven();
}

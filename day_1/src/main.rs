use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let filepath = "input.txt";
    let filepath = "input.txt";
    let vec_of_pairs = parse_file_to_vec(filepath)?;

    let sorted_first_column = extract_first_element_in_vector_of_pairs_into_new_vector_and_sort(vec_of_pairs.clone());
    let sorted_second_column: Vec<i32> = extract_second_element_in_vector_of_pairs_into_new_vector(vec_of_pairs.clone());

    let mut sorted_pairs: Vec<[i32; 2]> = Vec::new();

    for i in 0..sorted_first_column.len() {
        sorted_pairs.push([sorted_first_column[i], sorted_second_column[i]]);
    }

    // Print the first three pairs
    for i in 0..3 {
        println!("{:?}", sorted_pairs[i]);
    }

    let total: i32 = calculate_total_difference_in_array_of_numbers(sorted_pairs);

    println!("Total: {total}");
    Ok(())
}

// ##################################################
// Utility functions
// ##################################################

fn calculate_total_difference_in_array_of_numbers(input_vector: Vec<[i32; 2]>) -> i32 {
    let mut total = 0;

    for i in 0..input_vector.len() {
        let difference = input_vector[i][1] - input_vector[i][0];
        total += difference.abs();
    }

    return total;
}

fn parse_file_to_vec(path: &str) -> Result<Vec<[i32; 2]>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line?;               // Read the line
        let trimmed = line.trim();      // Trim leading/trailing whitespace

        if trimmed.is_empty() {
            continue;                   // Skip empty lines
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid file format: each line must contain exactly two numbers".into());
        }

        // Parse the two numbers as i32
        let x: i32 = parts[0].parse()?;
        let y: i32 = parts[1].parse()?;

        // Push them into a [i32; 2] array
        pairs.push([x, y]);
    }

    Ok(pairs)
}

fn extract_first_element_in_vector_of_pairs_into_new_vector_and_sort(input_vector: Vec<[i32; 2]>) -> Vec<i32> {
    let mut original_new_vector: Vec<i32> = Vec::new();

    for i in 0..input_vector.len() {
        original_new_vector.push(input_vector[i][0]);
    }

    let sorted_new_vector = sort_vector_of_numbers(original_new_vector);

    return sorted_new_vector;
}

fn extract_second_element_in_vector_of_pairs_into_new_vector(input_vector: Vec<[i32; 2]>) -> Vec<i32> {
    let mut original_new_vector: Vec<i32> = Vec::new();

    for i in 0..input_vector.len() {
        original_new_vector.push(input_vector[i][1]);
    }

    let sorted_new_vector = sort_vector_of_numbers(original_new_vector);

    return sorted_new_vector;
}

fn sort_vector_of_numbers(input_vector: Vec<i32>) -> Vec<i32> {
    let mut new_vector = input_vector;
    new_vector.sort();
    return new_vector;
}
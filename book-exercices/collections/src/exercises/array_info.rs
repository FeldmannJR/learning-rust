use core::f64;
use std::{collections::HashMap, io};

/**
 * Given a list of integers, use a vector and return the average value, median(when sorted, the value in the middle),
 * and mode (the value that occurs most often)
 */
pub fn read_and_process() {
    let cin = io::stdin();
    // Read array
    let mut input_array: Vec<i32> = Vec::new();
    println!("Please provide a valid number each line, when you're finished send -1");
    loop {
        let mut temp_buffer = String::new();
        cin.read_line(&mut temp_buffer)
            .expect("Error reading from terminal");

        let input_number: i32 = temp_buffer
            .trim()
            .parse()
            .expect("Please, type a valid number");

        if input_number == -1 {
            break;
        }
        input_array.push(input_number);
    }
    vector_info(&mut input_array);
}

fn vector_info(vec: &mut Vec<i32>) -> () {
    println!("Avg: {}", average(vec));
    println!("Median: {}", median(vec));
    println!("Mode: {}", mode(vec));
}

fn average(vec: &Vec<i32>) -> f64 {
    let sum = vec.iter().sum::<i32>() as f64;
    sum / vec.len() as f64
}

fn median(vec: &mut Vec<i32>) -> i32 {
    // Idk if there is a better way instead of doing a copy
    let mut sorted = vec.clone();
    sorted.sort();

    let len = sorted.len();

    sorted[len / 2]
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for number in vec {
        let cur_value = frequency.entry(*number).or_insert(0);
        *cur_value += 1;
    }
    // Probably theres a better way of doing this, but idk yet
    let mut max = (0, 0);
    for entry in frequency {
        if entry.1 > max.1 {
            max = entry;
        }
    }
    // returns key
    max.0
}

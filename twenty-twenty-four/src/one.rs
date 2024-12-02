use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() {
    println!("Hello from one.rs");

    let file = File::open("src/one/input");
    let reader = BufReader::new(file.unwrap());

    //  init array of integers
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut distance_list: Vec<i32> = Vec::new();
    let mut similar_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // split line by spaces
        let split = line.split("   ");

        let mut line_loop_count = 0;
        // iterate over split
        for s in split {
            // parse string to integer
            let num: i32 = s.parse().unwrap();
            println!("{}", num);

            if line_loop_count == 0 {
                // add to list1
                list1.push(num);
            } else {
                // add to list2
                list2.push(num);
            }

            line_loop_count += 1;
        }
    }

    // sort list1 by ascending order
    list1.sort();
    list2.sort();

    println!("List1 length: {:?}", list1.len());
    println!("List2 length: {:?}", list2.len());

    // loop over list1 and compare to list2
    for i in 0..list1.len() {
        let num1 = list1[i];
        let num2 = list2[i];

        // get distance between num1 and num2 but make sure it's positive
        let distance = (num1 - num2).abs();

        // add distance to distance_list
        distance_list.push(distance);

        // now, check for similar num1 in list2
        let mut sim_count = 0;
        for j in 0..list2.len() {
            let num_to_check_sim = list2[j];

            if num1 == num_to_check_sim {
                sim_count += 1;
            }
        }

        // calculate and add to similar list
        let calc_sim = num1 * sim_count;

        similar_list.push(calc_sim);
    }

    println!("Distance list: {:?}", distance_list);
    println!("Distance list length: {:?}", distance_list.len());

    // get sum of distance_list
    let sum: i32 = distance_list.iter().sum();
    let sum_similar: i32 = similar_list.iter().sum();
    println!("Sum: {:?}", sum);
    println!("Sum similar: {:?}", sum_similar);
}

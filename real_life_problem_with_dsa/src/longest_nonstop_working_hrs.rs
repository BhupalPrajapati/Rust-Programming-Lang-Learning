// Longest Non-stop work
// Description
// Given time slots numbers, we wan to determine the longest consecutive time slot
// tools
// HashSet, Vector and loops

// time slot of employees is
// the 1st slot is from 8 to 10 on Monday and the 2nd slot is 10 to 12 on the same day

use std::collections::HashSet;

// here we define a function to work the desired the functionality
fn longest_time_slot(working_slot: Vec<Vec<u8>>) -> usize {
    let mut employee_longest_nonstop_time: Vec<u8> = Vec::new();

    for i in &working_slot {
        employee_longest_nonstop_time.push(longest_period(i.clone()));
    }

    for (i, &period) in employee_longest_nonstop_time.iter().enumerate() {
        println!(
            "Employee Number {} has worked nonstop for {} slots",
            i, period
        );
    }

    let max_val = employee_longest_nonstop_time.iter().max().unwrap();

    employee_longest_nonstop_time
        .iter()
        .position(|&x| x == *max_val)
        .unwrap()
}

// for each employee i will calculate the longest period of her worl
// for that we need to define the another function

fn longest_period(working_slot: Vec<u8>) -> u8 {
    let mut longest_bg_period: u8 = 0;

    let slot_set: HashSet<_> = working_slot.into_iter().collect();

    for &slot in &slot_set {
        if !slot_set.contains(&(slot - 1)) {
            let mut current_slot = slot;
            let mut current_consecutive_slot = 1;

            while slot_set.contains(&(current_slot + 1)) {
                current_slot += 1;
                current_consecutive_slot += 1;
            }

            if current_consecutive_slot > longest_bg_period {
                longest_bg_period = current_consecutive_slot;
            }
        }
    }

    longest_bg_period
}

fn main() {
    println!("Starting");
    // define secdule of employees by using multidimensional vectors
    // inner vector is show the time slots in which employee may have work
    let schedule: Vec<Vec<u8>> = vec![
        vec![4, 1, 2, 5, 6, 10, 11],
        vec![3, 1, 2, 5, 7, 10, 11, 14],
        vec![3, 1, 15, 5, 13, 12, 10, 14, 15, 16, 17, 18, 8, 9],
    ];

    // here we print the longest_non-stop work of the employees

    println!(
        "Employees Number {} has the highest number of non-stop working slots",
        longest_time_slot(schedule)
    );
}

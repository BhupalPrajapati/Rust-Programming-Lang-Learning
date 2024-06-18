    // Employee with No Meetings
    // Desc 
      // -Given a meeting schedule of employees, we want to determine the overlaping time of employee


      // Tools
        // MultiDimensional Array and alot loops

use std::cmp;

// here we create a function is called function overlapping, which is calculate the overlapping time between two employeesq

fn overlaping_meeting(meeting_a:Vec<Vec<i32>>, meeting_b:Vec<Vec<i32>>)->Vec<Vec<i32>>{
    // here we create a empty vector for the 
    let mut intersection:Vec<Vec<i32>> = Vec::new();
    for i in 0..meeting_a.len() {
        for j in 0..meeting_b.len(){
            let(st_a, st_b) = (meeting_a[i][0], meeting_b[j][0]);
            let(end_a, end_b) = (meeting_a[i][1], meeting_b[j][1]);

            // creating the function overlap_status, where we can compare the varibale of the intersection and
            let overlap_status = overlap(st_a, st_b,end_a,end_b);
            if overlap_status!=None{
                intersection.push(overlap_status.unwrap());
            }
        }
    }
    intersection
}

// creating the new function to check these two meetings are overlapp or not

fn overlap(start_a:i32, start_b:i32, end_a:i32, end_b:i32)->Option<Vec<i32>>{
    // creating a empty varobale for now
     let mut intersection_time:Vec<i32> = Vec::new();
     if cmp::max(start_a,start_b) < cmp::min(end_a,end_b){
        intersection_time.push(cmp::max(start_a,start_b));
        intersection_time.push(cmp::min(end_a,end_b));
        Some(intersection_time)
     }else{
        None
     }
}

fn main(){
    
    // Declarning the starting time and ending time of the employee

    let meeting_a = vec![vec![13,15], vec![15,16], vec![7,9]];
    let meeting_b = vec![vec![14,15], vec![5,10]];

    // finally call the overlapping in the main function

    let intersection = overlaping_meeting(meeting_a, meeting_b);
    println!("The Overlapping Time are {:?}",intersection);
}
fn main(){
    
   /*                                     
let  student_db = vec![
    Student{
         name:"Bhupal".to_string(),
         grade :Some(32),
    },
    Student{
         name:"bhpal".to_string(),
         grade:Some(31),

    },
    Student{
         name:"Prajapat".to_string(),
         grade:None,
    }
];

// println!("{:?}",student_db );

let student_name = String::from("Bhu");
// let result_student = get_grade(&student_name, &student_db);

// This is not good/appricate method to define the math 
// instead of using match pattern we are using the if with the match pattern
// when we printing the value of none then this is not concvent ways for this that why we are using the 
// if let statament without the printing the none value  


// match result_student {
//     Some(grade)=>println!("grade is {grade}"),
//     None=>{},
// }
// if let Some(grade)=result_student{
//      println!("garde :{grade}");
// }

// check stustus of student
let student_status = check_grade_grade_student(&student_name, &student_db);

// This enum work with the math expresssion for the matching vslue in the expression

match student_status {
    Ok(option_grade)=>{
         //let student_grade = get_grade(&student_name, &student_db);
         // here we use our Some Varianrs for printing grade

         if let Some(garde)=option_grade{
              println!("Garde is :{garde}" );
         }
    }
    Err(error_msg) => println!("{error_msg}"),
   
}
}
struct Student{
    name:String,
    grade:Option<u32>,
}

// fn get_grade(student_name:&String,student_db:&Vec<Student>)->Option<u32>{
//      for student in student_db  {
//          if student.name == *student_name {
//              return student.grade;
//          } 
//      }
//      None
// }

fn check_grade_grade_student(student_name:&String, student_db:&Vec<Student>)->Result<Option<u32>,String>{
          for student in student_db  {
          if student.name == *student_name {
             return Ok(student.grade);
         }
    }
    Err(String::from("Student is Not found"))

    */

let x = 10;

                                        // Normal practicse of question by using the Resut enum 

       let number = 7;
       if let Err(e) = calculate_area(number){
        println!(" Erros is:{e}");
       }                                 
}

fn calculate_area(num:i32)->Result<i32,String>{
    if num >= 0{
        let result = num*num;
        println!("The result {} is : {}",num,result );
        return Ok(result)
    }
    Err("Negative Number is Provided".to_string())
}


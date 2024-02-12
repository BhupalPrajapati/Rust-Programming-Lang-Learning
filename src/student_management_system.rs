/*use std::collections::HashMap;

struct Student{
    id :i32,
    name : String,
    grade : String,
}
struct StudentManager {
    students:HashMap<i32,Student>
}

impl StudentManager {
    fn new()-> Self {
        StudentManager{
            students:HashMap::new(),
        } 
    }
    fn add_student(&mut self, student: Student) -> Result<(), String>{
        if self.students.contains_key(&student.id){
            Err(format!("Student with {} is alreday is exist",student.id))
        }else {
            self.students.insert(student.id, student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student>{
        self.students.get(&id);
        None
    }
    
}
fn main(){
    let mut manager = StudentManager::new();
    let student1= Student{
         id:1,
         name:String::from("Name"),
         grade:String::from("A"),
    };

    let student2 = Student{
        id:2,
        name:String::from("Student 2"),
        grade:String::from("B"),
    };

    manager.add_student(student1).unwrap();
    manager.add_student(student2).unwrap();
    if let Some(student) = manager.get_student(1) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
    if let Some(student) = manager.get_student(2) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }

}*/

/* 

use std::collections::HashMap;
struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(format!("Student with ID {} already exists", student.id))
        } else {
            self.students.insert(student.id, student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.students.get(&id)
    }
}

fn main() {
    let mut manager = StudentManager::new();

    let student1 = Student {
        id: 1,
        name: String::from("Alice"),
        grade: String::from("A"),
    };
    let student2 = Student {
        id: 2,
        name: String::from("Bob"),
        grade: String::from("B"),
    };

    manager.add_student(student1).unwrap();
    manager.add_student(student2).unwrap();

    // Retrieve and print student information
    if let Some(student) = manager.get_student(1) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
    if let Some(student) = manager.get_student(2) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }

}
    */



    use std::collections::HashMap;
    struct Student {
        name: String,
        age: i32,
        grade: String,
    }
    
    fn add_student(
        student_database: &mut HashMap<i32, Student>,
        id: i32,
        name: String,
        age: i32,
        grade: String,
    ) {
    
        let entry = Student{name,age,grade};
          if student_database.contains_key(&id) {
              println!("The key aldeay exist")
          }else {
              student_database.insert(id, entry);
          }
        }
    
    
    fn main() {
        let mut student_database: HashMap<i32, Student> = HashMap::new();
        add_student(
            &mut student_database,
            1,
            String::from("John"),
            17,
            String::from("Grade 11"),
        );
    
        add_student(
            &mut student_database,
            2,
            String::from("Sarah"),
            16,
            String::from("Grade 10"),
        );
    
        // Printing the student database
    
        for (id, student) in &student_database {
            println!("Student ID: {}", id);
            println!("Name: {}", student.name);
            println!("Age: {}", student.age);
            println!("Grade: {}", student.grade);
            println!("------------------");
        }
    }
    
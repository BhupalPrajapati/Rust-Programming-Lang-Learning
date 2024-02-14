struct Employee{
    name:String,
    salary:u16,
}
struct Employee_Records{
    employee_db : Vec<Employee>,
}

// we explicity implement the iterstor

impl Iterator for Employee_Records {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len()!=0 {
            // Then we grap the name of the employee at first index

            let result = self.employee_db[0].name.clone();

            // then remove the entry from the vector
            self.employee_db.remove(0);
            Some(result)
        }else {
            None
        }
    }
}
fn main(){
    let mut emp1 = Employee{
        name:"manxe".to_string(),
        salary:32,
    };

    let mut emp2 = Employee{
        name:"Ram".to_string(),
        salary:23,
    };
    let mut emp_db = Employee_Records{
        employee_db:vec![emp1,emp2],
    };
//     println!("{:?}",emp_db.next());
//     println!("{:?}",emp_db.next());
//     println!("{:?}",emp_db.next());
//     println!("{:?}",emp_db.next());


// The for loops know how to handle the type that implement the itertors 
// The variable stringg is item of the employee
for employee in emp_db  {
println!("{employee}");    
}
 }
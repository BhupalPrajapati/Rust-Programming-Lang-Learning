#[derive(Default)]
struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
    salary: u8,
    nationality: u8,
}
impl Student {
    fn some_fn1(&self) -> String {
        todo!()
        // TODO: Implement the logic here
    }
    fn some_fn2(&mut self) -> String {
        todo!()
    }
}
fn main() {
    let mut student = Student::default();
    student.name_std = String::from("John Doe");
    student.age = 20;
    student.sex = 'M';
    student.country = String::from("USA");
    student.salary = 50000;
    student.nationality = 1;
}

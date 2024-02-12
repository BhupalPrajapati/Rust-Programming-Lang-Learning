
// defn of generic is used to allow in struct enum, function and there places holder of data


struct Point<T,U>{
    name : T,
    age : U
}

// implementation of 

impl<T,U> Point<T,U> {
    fn new(name:T, age:U)->Point<T,U>{
        Point{name,age}
    }
}

impl Point<i32,i32> {
    fn printing(&self){
        println!("The print of the value is {}, {}",self.name,self.age );
    }
    
}

impl Point<i64,i64> {
    fn printing(&self){
        println!("{},{}",self.name,self.age );
    }
    
}
// imple another method 

fn add_number<T,U>(result1:&Point<T,U>,result2:&Point<T,U>){
    unimplemented!();
}
fn add_numbers<T,U>(result:&Point<T,U>,result2:&Point<T,U>){
    unimplemented!();
}
fn main(){
         let result: Point<i32,i32> = Point::new(1, 2);
         let result1 = Point::new(2.0, 1.0);
         let result2 = Point::new(12, 2);
         // access the printing function inside the main method
        result.printing();
        // result1.
        add_number(&result,&result);
        add_numbers(&result,&result2);


}
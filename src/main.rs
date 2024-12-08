// mod student_management_system;
// mod Result;
//
// use std::{collections::{btree_set::Intersection, HashMap}, os::windows::raw::HANDLE};

// here we can used the array tool for intersection
// use array_tool::vec::*;
// use the Modules of customer, order, and product
mod Box_pointer;
/// this is the moule structure
// mod customer;
// mod order;
// mod product;
// use practicse::{Customer,Product,Order};
mod closures;
mod combinators;
mod generic;
mod into_Iterator;
mod iterating_over_collection;
mod iterating_through_option;
mod iterator;
mod lifetime;
mod linkedList;
mod marker_derived_trait;
mod refcell_pointer;
mod traits;

fn main() {
    // println!("Hello, world!");
    //  let x: [i128;5]=[1,2,3,4,5];

    //  let x = vec![1,2,3,4,5];

    //     println!("{:?}",x);
    // panic!("{:?}",x );

    // let x:[i64;3]=[1,2,3];
    // print!("{:?}",x);
    // str and string in rust

    let mut x: &str = "nshd";

    println!("{}", x);

    // let mut y: String = String::from("the value i");
    // y.push('s');
    // println!("{}",y);
    // defining the vector compound types of data

    // let a: Vec<i32>=vec![1,2,3,4];
    // let num = a[3];
    // println!("{}",num);

    // Tuples
    // Tuples containes more than two types of value for eg integers and string;
    //  let info:(&str, i32, &str, i32)=("Id",41,"section",3);
    //  let x: &str = info.0;
    //  println!("{}",x);

    // print the books details by using the tuples

    // type  book = (String,String,u32);
    // // 1st book details
    // let book1: book=(String::from("Rust Programming Lang"), String::from("Bhupal"),2015);
    // println!("Book name:{}, Author:{}, year:{}",book1.0,book1.1,book1.2);

    // // 2nd book details

    // let book2: book = (String::from("Java Programming Lang"), String::from("Java Author"),1998);
    // println!("Book name:{},Author:{},Year:{}",book2.0,book2.1,book2.2);

    //function and blocks
    // my_name("This is function def");

    // fn my_name(s: &str){
    //     println!("{}",s);
    // }

    // 1) function also return the some values for eg
    // one point in the function we can not retunr multiple return ata time

    // multiply(2,3);

    // fn multiply(num1: i32 , num2: i32)->i32{
    //     println!("The multiplication is" );
    //     let num = num1*num2;
    //     println!("{}",num);

    // }

    // multiply of two number
    //    let print =multiple(12, 12);

    //     fn multiple(num1:i32,num2:i32) ->(i32,i32) {
    //         num1*num2;

    //     }

    // Code blocks in rust
    //       let fullname: String= {
    //           let first_name = "Bhupal";
    //           let second_name = "Prajapati";
    //              format!("{first_name} {second_name}");
    // };
    // println!("{}",fullname);

    // if else statement

    // let num = 51;
    // if num<50{
    //    println!("the number is less than 50");
    // }
    // else {
    //     println!("The number is greater than 40");
    // }

    // match conditional statement

    // let grade=match marks {
    //       90..=100=>'A',
    //       80..=90=>'B',
    //       70..=79=>'C',
    //       __=>'F',
    // };
    // println!("{}",grade);

    // control flow statement in the rust
    // let vec=vec![12,23,21,1];
    // for i in vec {
    //     print!("{i}");
    // }

    // let mut n: String = String::new();
    // std::io::stdin()
    // .read_line(&mut n)
    // .expect("Invailed in")

    // How to take input from the user in rust

    //   let mut n: String = String::new();
    //   std::io::stdin().read_line(&mut n).expect("failed to compile");
    //   let n: f64=n.trim().parse().expect("invalid input");
    //   println!("{n}");

    // using of static and constaant keyowrd in rust

    // static keyword is used with the varable which takes the space in the memory

    //  static WELCOME: &str = "Welcome to you";
    //  println!("{WELCOME}");
    //  let x = WELCOME;
    //  println!("{x}");

    //  //

    //  const PI:f64 =3.14;
    //  let p = PI;
    //  println!("{p}");

    // Ownership basics

    //   let s: String = String::from("World");
    //   {
    //   let s1 = s;
    //   }
    //   println!("{s1}");

    // for premitive data

    // let x = 34;
    // {
    // let y = x;
    // }
    // println!("{y}");

    // Vector type data type

    //  let vec1 = vec![1,2,3,4];
    // //  let vec = vec1.clone();
    // //  println!("{:?}",vec1);
    // owership_takes(vec1.clone());
    //       println!("{:?}",vec1);

    // fn owership_takes(vec: Vec<i32>){
    //       println!("{:?}",vec);
    // }

    // // return value from the owernship

    // fn tran_own()->Vec<i32>{
    //       vec
    // }

    // let vec_1 = vec![1,2,3,4];
    // // makes_own(vec_1.clone());
    // println!("{:?}",vec_1);
    // makes_own(vec_1)
    // }
    // fn makes_own(vec:Vec<i32>){
    //       println!("{:?}",vec);

    //// ownership is out of the function that is also called the return type owernship function

    //  let vec_2 = gives_owner();
    //  println!("{:?}",vec_2);
    //  let x = com_bin(vec_2);
    //  println!("{}",x);

    //  let x = 10;
    //  st(x);
    //  println!("the main x is :{x}")
    // }

    //  fn gives_owner()->Vec<i32> {
    //       vec![1,2,2,4]

    //  }

    //  fn com_bin(mut vec: Vec<i32>)->Vec<i32>{
    //       vec.push(10);
    //       vec
    //  }
    // lets ecplore the statoc type varibale in the rust

    //  fn st(mut var:i32){
    //       var = 56;
    //       println!("in fun, var is{var}");

    // let name: String = String::from("Bhupal");
    // let sirname: &str = "Prajapati";
    // // call the function
    // print(name.clone(),sirname);
    // println!("{} {}",name,sirname);
    //  }

    //  fn print(a1:String,a2:&str){
    //       println!("{} {}",a1,a2);

    // owenrship concept in a loop

    // let mut my_vec = vec![1, 2, 3, 4, 5];
    //     let mut temp;

    //     while !my_vec.is_empty() {
    //         temp = my_vec.clone(); // Something wrong on this line
    //         println!("Elements in temporary vector are: {:?}", temp);

    //         if let Some(last_element) = my_vec.pop() { // pop() is used to remove an element from the vec
    //             println!("Popped element: {}", last_element);
    //         }
    //     }
    // let str1={
    //       let str1 = generate_string();
    //       str1
    // };

    // let _str2 = str1;   // Something wrong with this line
    // }

    // fn generate_string() -> String {
    // let some_string = String::from("I will generate a string");
    // some_string

    // Borrowing rules in the rust

    // there is a 2 rules in the borrowing concepti in the rust
    // 1) at a time you can have one mutalbe refernces or any immutable refernces
    // 2) any references i vaild

    //   let mut  var = vec![1,2,3];
    //   let x = & var;
    //   let y = & var;
    //   let z =& var;

    //   println!("{:?} {:?} {:?} ",x,y,z);
    //   let a = &mut var;
    //   println!("{:?}",a);

    //   let var2={
    //       let var1 = vec![1,2,3];
    //        &var1 // this is called dangling pointer
    // };
    /*
    let mut var = vec![1,2,3,4,4];
    // let ref_1 = & var;
    let ref_2 = &mut var;
        makes_own(&ref_2);
        println!("{:?}",ref_2);

        // call the function into main function
         takes_own(ref_2);
        println!("{:?}",var)

     }

     fn makes_own(vec:&Vec<i32>){
          println!("{:?}",vec);

     }

     // this function take the ownership of the vector and update value and simply return is

     fn takes_own(vec:&mut Vec<i32>){
          vec.push(10);

     }
     fn borro_val()->&Vec<i32>{
          let c = vec![1,2,3,4];
          &c
          */

    // derefering the value in rust
    // derefereing is the process of accessing the value of pointed to by a refere

    //  let mut some_name = 40;
    //  let ref_1 = &mut some_name;
    //  let def_copy = *ref_1; // derefering donn with the helpn of *
    //  *ref_1 = 13;
    //  println!("{some_name},{def_copy}")

    // we know that the scalar data types are stored in the stack not in the heap. so we have no need to
    // copy the varible to another varibale

    //     let mut  var = vec![1,2,3,4];
    //     let ref_1 = &mut var;
    //     let copy_1 = ref_1.clone();
    //     *ref_1 = vec![1,2];
    //     println!("{:?}",ref_1);

    // let mut some_vec = vec![1, 2, 3];
    //     let mut first = get_first_element(&some_vec);
    //     some_vec.push(4);
    //     println!("The first number is: {}", first);
    // //     some_vec.push(4);
    // }

    // fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
    //     &num_vec[0]

    // let mut vec_1 = vec![1, 2, 3];
    //     let vec_2 = vec![4, 5, 6];
    //     let mut vec_ptr: &Vec<i32>;
    //     vec_ptr = &vec_1;
    //     println!("vec ptr is pointing to vec_1");
    //     vec_ptr = &vec_2;
    //     println!("vec ptr is updated and now pointing to vec_2");

    // let mut first_num = 42;
    //     let mut second_num = 64;
    //     let ref1 = &mut first_num;
    //     let mut ref2 = &mut second_num; // a mutable references means that the reference can be updated to point to some other variable

    //     *ref1 = 15;
    //     *ref2 = 10;
    //     ref2 = ref1;

    //     println!("Updated first number: {ref2}");

    // struct keywords is used to define data in one places
    // let mut my_car = Car{
    //      owner: String::from("Bhupal"),
    //      year:1998,
    //      fuel_level:43.6,
    //      price:32344,

    // };

    // println!("{}",my_car);
    /*let car_year = my_car.year;
    println!("{}",car_year);
    my_car.fuel_level = 30.2;

    let extrac_owner = my_car.owner.clone();
    println!("{}",my_car.owner);

    // let make another varibale to access the value of car in new car instances

    let makes_own = Car{
         // owner:"new name".to_string(),
         ..my_car  // by using two dot we can access the instanes of the struct variable
    };
    println!("{}",makes_own.owner);


     }
     struct Car{
         owner : String,
         year : u32,
         fuel_level : f32,
         price : u32, */

    // Tuples
    // let point_2D = (1,2);
    // let point_3D = (1,2,3);
    // struct point_2D(i32,i32);
    // let point = point_2D(1,2);

    // let mut red = Color(255,12,43);
    //  red.2 = 10;
    // println!("red is {} {} {}",red.0,red.1,red.2);

    // }
    // // this is the tuples which is not define licke strut
    // struct Color(u8,u8,u8);

    // fn dispaly_color(Color:Color){
    //      println!(" {} {} {}",red );
    //   let mut _com = Color{red :1, blue:2, green:3};
    //   _com.dispaly_color();

    // }
    // struct Color{
    //      red :i32,
    //      blue :i32,
    //      green :i32,
    // }
    // impl Color{
    //      // define the asscoative function which
    //      fn insurrence()->u32{
    //           22
    //      }
    //      fn selling_price(&self)->u32{
    //           Color::insurrence()

    //      }
    //      fn dispaly_color(&self){
    //           println!("red:{} blue:{} green:{}",self.red,self.blue,self.green );
    //      }

    // }

    // fn reused(&mut self, gallon:f32){
    //      self.r

    // let mut day = "sunday".to_string();
    // // println!("{}", day);

    // let  weekday = vec![
    //      "monday".to_string(),
    //      "Tuesday".to_string(),
    //      "Wednesday".to_string(),
    //      "thursday".to_string(),
    //      "Friday".to_string(),
    //      "saturday".to_string(),
    //      "sunday".to_string()
    // ];

    // day = weekday[1].clone();
    // println!("{}",day );

    // let day = Weekday::Friday;
    // println!("{}",day );
    // }
    // impl Weekdays {

    // enum Weekday {
    //      Monday,
    //      Tuesday,
    //      Wednesday,
    //      Thursday,
    //      Friday,
    //      Saturaday,
    //      Sunday,

    // }
    //  let participat = Traveltype::Train(50.0);
    //  println!("{}",participat.tarvel_allowance() );

    // }
    // enum Traveltype {
    //      Car(f32),
    //      Train(f32),
    //      Plan(f32),

    // }
    // impl Traveltype {
    //     fn tarvel_allowance(&self)->f32{
    //      let allowance =match self {

    //          Traveltype::Car(miles) => miles * 2.0,
    //          Traveltype::Train(miles) => miles * 3.0,
    //          Traveltype::Plan(miles) => miles * 4.0
    //      };
    //      allowance
    //     }

    /*  This is Program to libray Management System
    let rust_book = Item{
         id:1,
         title:"Rust Book".to_string(),
         year:2010,
         type_:ItemType::Book,
    };

    // print!("........");
    let rust_magazin = Item{
         id:2,
         title:"Rust Magazin".to_string(),
         year:2011,
         type_:ItemType::Magazine,

    };

    display_item_info(&rust_book);
    display_item_info(&rust_magazin);

    }
    struct Item{
         id:i32,
         title:String,
         year:u32,
         type_:ItemType,

    }
    #[derive(Debug)]
    enum ItemType {
         Book,
         Magazine,

    }
    fn display_item_info(item:&Item){
         println!("Book Id:{:?}",item.id );
         println!("Book Title:{:?}",item.title );
         println!("Publication Year:{:?}",item.year );
         println!("publication Type:{:?}", item.type_);
         */

    // Here we used the Option enum
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

    let student_name = String::from("Bhupal");
    let result_student = get_grade(&student_name, &student_db);

    // This is not good/appricate method to define the math
    // instead of using match pattern we are using the if with the match pattern
    // when we printing the value of none then this is not concvent ways for this that why we are using the
    // if let statament without the printing the none value


    // match result_student {
    //     Some(grade)=>println!("grade is {grade}"),
    //     None=>{},
    // }
    if let Some(grade)=result_student{
         println!("garde :{grade}");
    }


    }
    struct Student{
         name:String,
         grade:Option<u32>,
    }

    fn get_grade(student_name:&String,student_db:&Vec<Student>)->Option<u32>{
         for student in student_db  {
             if student.name == *student_name {
                 return student.grade;
             }


         }
         None

         */

    // problem1: printing the 1st letter of alphabet

    /*
       let my_chars = vec![];
      match find_alph(&my_chars)  {
          Some(result)=>println!(" The Alphabet is :{result}"),
          None=>println!("Emoty" ),
      }
    }

    // make a function who find the index of the alphabet

    fn find_alph(chars:&Vec<char>)->Option<char>{
        if chars.len()>0{
         Some(chars[0])
        }else {
            None
        }
        */

    // problem2 : Matching the fruit name with help of the option enum
    /*

       let user_fruit = String::from("Apple");
       if let Some(fruit)=check_fruit(user_fruit){
         println!(" Matching Fruit :{fruit}");
       }
    }

    // define the check_fruit name which contains list of fruit

    fn check_fruit(input_fruit:String)->Option<String>{
         let fruit_data = vec![
              String::from("Apple"),
              String::from("banana"),
              String::from("Mango")
         ];

         for fruit in fruit_data {
              if input_fruit==fruit {
              return Some(fruit);
              }
         }
         None


    // */
    // let mut x:[i32;5] = rm_y(1,2,3,4,5);

    // }
    // fn rm_y(x:[i32;5]){
    //      rm_y=

    // let arr:[i32;5] = [1,2,3,4,5];
    //   println!("array is {:?}",arr);
    //   println!("array size is :{}",arr.len());

    //   for val in arr.iter(){
    //      println!("value is :{}",val);
    //   }

    // }

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

    // Implementing The HashMap
    /*
    let mut person:HashMap<&str,i32> = HashMap::new();
      person.insert("Bhupal",12);
      person.insert("Raj",2);
      println!("value is :{}",person.get("Bhupal").unwrap());
      println!("{}",person.contains_key("Bhupal"));

      if person.contains_key("Raja"){
          println!("The value is Exits")
      }else {
          println!("Value is Not Exist")
      }

      match person.get("raj"){
          Some(value) => println!("value Exist:{}", value),
          None => println!("The value is Not exits"),
      }

      // we are going to use of references of value , the references wii be ended when the loop is end

      for (name,age) in &person {
          println!("The person {} has age :{}",name,age);
      }

      */

    // Another Example of hashmap
    /*
       let mut person :HashMap<&str,&str> = HashMap::new();
    //    person.insert("Bhupal", "Prajapati");
    //    person.insert("Bhupal", "Kohar") ;
    //    println!("{:?}",person);

         // Here we are using the entry function who check the value is already exist or not

         person.entry("Bhupal").or_insert("Prajapati");
         person.entry("Bhupal").or_insert("Kohar");
         println!("{:?}",person)

         */

    //     let some_vec = vec![1,2,3,4,5,6,7,8];
    //     let mut freq_vec:HashMap<i32,u32> = HashMap::new();

    //     for i in &some_vec  {
    //         let fre = freq_vec.entry(*i).or_insert(0);
    //         *fre+=1;
    //     }
    //     println!("{:?}",freq_vec );
    // //     let Intersection= Intersectio??

    // Choose Between Associated Type and Generic type
    /*
          let p1 = Point{x:1,y:2};
          let p2 = Point{x:2,y:1};
          let p3:Point = p1.add(p2);
          assert_eq!(p3.x,3);
          assert_eq!(p3.y,3);

          let p3= Point{x:1,y:1};
          let p4 = p3.add(2);
          assert_eq!(p4.x,3);
          assert_eq!(p4.y,3);

    }

    trait Addition<Rhs,Ouput> {
    //     type Rhs;
    //     type Ouput;
        fn add(self, rhs:Rhs)->Ouput;
    }

     struct Point{
         x:i32,
         y:i32,
     }
    impl Addition<Point,Point> for Point{
         // type Rhs  = Point;
         // type Ouput = Point;
         fn add(self, rhs:Point)->Point {
             Point{
              x:self.x+rhs.x,
              y:self.y+rhs.y,
             }
         }
    }

    impl Addition<i32,Point> for Point {
    //     type Rhs = i32;
    //     type Ouput = Point;
        fn add(self, rhs:i32)->Point{
            Point{
              x:self.x+rhs,
              y:self.x+rhs,
            }
        }
    }

    struct Line{
         start:Point,
         end:Point,
    }
    impl Addition<Point,Line> for  Point{
         fn add(self, rhs:Point)->Line {
             Line{
              start:self,
              end:rhs,
             }
         }

    }
    */

    // Here we implement the polymorphsim in rust

    let car = Sedon;
    car.drive();
    road_trip(&car);
    //     let electric = SUV;
    //     electric.drive();
    //     road_trip_suv(&electric);
}
struct Sedon;
impl Landcable for Sedon {
    // fn drive(&self){
    //      println!("sedon is driving")
    // }
}
struct SUV;
impl Landcable for SUV {
    //     fn drive(&self){
    //      println!("Suv is driving");
    //     }
}
fn road_trip(vehicle: &impl Landcable) {
    vehicle.drive();
}

trait Landcable {
    fn drive(&self) {
        println!("Defaut implementation")
    }
}

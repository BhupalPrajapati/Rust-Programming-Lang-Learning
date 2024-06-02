// use array_tool::vec;

// use lifetime::rand;
// use rand::Rng;
fn main(){
    
        // here is some stack type data which is not stored in heap so that we can refernces or moved varibale one to another variable

//     {
//     let i = 21;
//     }
//     let j = i;
//     println!("{}",i );



              // Here we are going the practicse of lifetime of a varible thorugh the youtube video
/* 
// let some_vl = 10; // this is stack based life times so does not need explicit define the life of varibale the compiler is automatically care abhout this
let some_vl = String::from("Manxe");
// println!("{}",some_vl);
let one_vl = String::from("Bhupal");

let result = gen_f(&some_vl,&one_vl);
println!("{}",result);  
// println!("{}",some_vl);            
 }  
    fn gen_fd<'a>(param_1:&'a str,param_2:&'a str)->&'a  str{
      // println!("{}", param_2);
    //  param_1;
    //  param_2

    if param_1>param_2{
      param_1
    }else {
        param_2
    }
    } 

  fn gen_f<'a>(param_1:&'a i32, param_2:&'a i32)->&'a i32{
    if param_1>param_2{
      param_1
    }else {
        param_2
    }
  }   */


/* 
  let some = vec![1,2,3,4];
  let val = vec![2,3,4];
  let result = examp(&some,&val);
  println!("{:?}",result);
}        

         // Another example of life time

 fn examp<'a>(param_1:&'a [i32],param_2:&'a [i32])->&'a [i32]{
  if param_1.len()>param_2.len(){
    &param_1[0..2]
  }
  else {
      &param_2[0..2]
  }
 }   
*/


          
          
   
   
   /*               
   
                     // Another example of life time of variable who takes the both heap type and stack type data
// float type data
let vl1 = 1.1;
let vl2 = 2.1;
let result1 = find_sml(&vl1, &vl2);   
println!("{:?}",result1);        

// string type data

let s1 = String::from("manxe");
let s2 = String::from("Bhupal");
let result2 = find_sml(&s1, &s2);
println!("{}",result2);
}            

fn find_sml<'a, T:std::cmp::PartialOrd>(param_1:&'a T, param_2:&'a T)->&'a T{
  if param_1 < param_2{
    param_1
  }else {
      param_2
  }
}

   */                 
                    // Take example of heap type data

//  let str1 = String::from("Voli");
//  str_fn(str1);
//  let str2 = &str1;
//  println!("{str1}");    


                    // Dangaling References

// let i;
// {
//     let f= 32;
//     i = &f;
//     println!("{}",i );



                      // Non-lexial life times










    //   let mut vec_1 = vec![1,2,3,4];
    //   let ref_1 = &vec_1;
    // //   let ref_2 = &mut vec_1;
    //   println!("{:?}",ref_1 );
    //   let ref_2 = &mut vec_1;
    //   ref_2.push(3);
    //   println!("{:?}",ref_2);










                    // generic type life times


/*                     let int1 = 32;
//  let picked_value;
 {
 let int2 = 21;
 let picked_value = picked(&int1, &int2);
 println!("{picked_value}"); 

 }     
//  println!("{picked_value}");              

        
}
// println!("{}",i );                    
              


// try the ownership for the function

// fn str_fn(s:String){
//     print!("{}",s );
// }

     // function for the picked value in the in life times

     

fn picked<'a>(i:&'a i32,j:&i32) -> &'static i32{
//         if rand::random(){
//                 i
//         }else{
//                 j
//         }

     // the function is always return the refernce of i

//      i
       
       // static life time(if you want to alive a variable for function life time then used static)

//        let x = 30;
//        &x  (this is gives an error)
  let y:&'static i32 = &6;
  y
}     */


let mut some_data = Addnm{data:&[1,2,3,4]};
// println!("{}",some_data);
let previous_date = some_data.data(&[3,2,1,4]);
println!("{:?}",previous_date);
println!("{:?}",some_data.data);

}

struct Addnm<'a>{
  data:&'a [i32],
}
// write imple block for the 

impl<'a> Addnm<'a>{
  // we are write to find the new data and return thr previous data   
// after adding the lifetimes of new_datw(which comes from the instance of struct) then code is fine bcz no need to define explicitly life times in output of reference bcz it is automatically done with lifetimes of elisions
 

  // fn data<'b>(&'b mut self, new_data:&'a [i32])->&'b [i32]{
    // after follow the rule of life time ellision, it has main three rules
    // 1)if each parameter that is a reference gets its own life time parameters annotations
    // 2) if there is exactly one input life time parameter that life time is assigned to all output life time parameters
    // 3) if there are multiple &mut or &mut self, the life time of self is assigned to all ouput life time paramteres
    fn data(&mut self, new_date:&'a [i32])->&[i32]{
    let previous_date = self.data;
    self.data=new_date;
    previous_date
  }
}
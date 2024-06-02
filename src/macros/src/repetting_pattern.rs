
/* 
macro_rules!repetting_macro{
    ()=>{
           // create empty string
           String::now();
    };
}
fn main() {
      let str_mul = repetting_macro!();
}

*/





//Consider the code below. Show the expansion part of this code, espacially for the invocation to macro.
/* 
macro_rules! make_functions {

    ($($func_name:ident: $return_type:ty => $return_expr:expr),+) => {

        $(

            fn $func_name() -> $return_type {

                $return_expr

            }

        )+

    };

}



make_functions!(foo: i32 => 42, bar: String => "hello world".to_owned());



fn main() {

    let result1 = foo();

    let result2 = bar();

    println!("foo result: {}", result1);

    println!("bar result: {}", result2);

}
*/


macro_rules! sum_macro {

    ($($x:expr),*) => {

        {

            let mut sum = 0;

            $(sum += $x;)*

            sum

        }

    };

}



fn main() {

    let result = sum_macro!(1, 2, 3, 4, 5);

}




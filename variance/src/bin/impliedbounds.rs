// implied bounds

// life time of bounds are some impled, means we did not need to explicitly write , compiler will automatically add them

struct GenricBounds<'a, T> {
    b: &'a T,       // 'a:'b => 'a outlives 'b 
}


fn lifetime_bound<'a,T>(x:GenricBounds<T>){} // here 'a is not mentioned, but it is there, it is implied

fn lifetime_bound1<'b,U>(x:&'b U){} // here 'a is not mentioned, but it is there, it is implied


fn no_lifetime_bound<'a,T>(){} // here 'a is not mentioned, but it is

trait Trait<'a> {
    fn method(&self);
}

impl<'a> Trait<'a> for GenricBounds<'a, i32> {
    fn method(&self) {
        println!("Trait method");
    }
}
fn main() {
    println!("Hello, World!");
}

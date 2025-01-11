// we have following function, we need to insert some refernce so that we can compile and it give us greater one

pub mod subtypingandvariance;

// compile unable to resolve the life time of the output based on the lifetime elision rules
//Life time elision rules
// 1. each parameter that is a reference gets its own lifetime parameter
// 2. there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. when there are mutable or immuatable ref to self.
fn largest<'a, 'b>(x: &'a u8, y: &'b u8) -> &'a u8
where
    'b: 'a,     // 'b:'a -> 'b outlives 'a last atlast as long as 'a 
{
    if x > y {
        x
    } else {
        y // y is
    }
}

fn main() {
    println!("{}", largest(&10, &20));
}

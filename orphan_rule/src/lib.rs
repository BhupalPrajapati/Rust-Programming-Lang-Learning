// in this lib.rs file we define the various struct
pub struct Point {
    pub x:i64,
    pub y:i64,
}
// make a method which print the point 
pub trait Printablr {
    fn print(&self);
    
}

// the orphan rule is bit difference in case of generic
pub trait Addition<T,U> {
    fn add(self,rhs:T) -> U;
    
}


//  lets implement the printable trait in this main method
// bring the both print and printable into the scope
 use orphan_rule::{Point,Printablr,Addition};

 // for the orphan rule we need to create a wrapper struct
 struct PointWrapper(Point);  // this is tuple struct(in this case) because the poitn wrapper is defined within current grade , we can implement the printable trait on it
 impl Printablr for PointWrapper {   // this is gives the error because it is not follow the orphan_rule
      fn print(&self) {
        println!("({}, {})", self.0.x, self.0.y);  // to get the access of 1st element of tuple we use self.0.x
      }
     
 }

 // addition is a point which is result in point add the same x axia and y axia
// impl Addition<Point,Point> for Point {  // this is compiled because it is volating the orphan rule
//     fn add(self,rhs:Point)->Point {
//         Point {
//             x:self.x + rhs.x,
//             y:self.y + rhs.y,
//         }
//     }
// }

// lets create the another struct which is not defined in the current crate
struct Line{
    pub start:Point,
    pub end:Point,
}
// now implement the addition function
impl Addition<Point,Line> for Point {   // this iscompiled because one of the generic is defined which is locally in used
     fn add(self, rhs:Point) -> Line {
         Line {
             start:self,
             end:rhs,
         }
     } 
}

// trying to implement the i32
impl Addition<i32,i32> for i32{  // this is also not compiled bacuasue it is not defined in locally it is defined in the libray
                                 // the requirements of at least one type should atleast defined locally
    fn add(self, rhs:i32) -> i32 {
        self + rhs
    }
}


fn main() {
    println!("Hello, world!");
}

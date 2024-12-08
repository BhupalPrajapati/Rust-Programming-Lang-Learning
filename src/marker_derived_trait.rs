// use std::clone;

// this is super trait with the
/*
trait Propertites: PartialEq + Default + Clone {}


#[derive(Debug, PartialEq,Default,Clone)]

struct Student {
    name: String,
    age: u8,
    sex: char,
}
impl Propertites for Student {}

fn main() {
    let student1 = Student {
        name: String::from("Bhupal"),
        age: 22,
        sex: 'M',
    };
    let student2 = Student {
        name: String::from("Prajapati"),
        age: 23,
        sex: 'M',
    };

    println!("Student :{:?}", student2);
    println!("Student is equal is : {}", student1 == student2);
}
*/

// I want to calculate the how much far my car travel within the 3 hours
#[derive(Debug)]
struct Km {
    value: u8,
}
#[derive(Debug)]
struct Kmh {
    value: u8,
}
#[derive(Debug)]
struct Miles {
    value: u8,
}
#[derive(Debug)]
struct Milesh {
    value: u8,
}

//Here we implementaion of struct by one by oine

// impl Kmh {
//     fn distance_in_three_hour(&self) -> Km{
//         Km{
//             value:self.value*3,
//         }
//     }
// }

// impl Milesh {
//     fn distance_in_three_hour(&self)->Miles{
//         Miles{
//             value:self.value*3,
//         }
//     }
// }

// Now we are using the trait to capturing the common behaviours of function
trait DistanceThreeHour {
    type Distance;
    fn distance_in_three_hour(&self) -> Self::Distance;
}

impl DistanceThreeHour for Kmh {
    type Distance = Km;
    fn distance_in_three_hour(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHour for Milesh {
    type Distance = Miles;
    fn distance_in_three_hour(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

fn main() {
    let speed_kh = Kmh { value: 12 };
    let disrance_in_km = speed_kh.distance_in_three_hour();
    println!(
        "At:{:?}, you travel {:?} in 3 hours",
        speed_kh, disrance_in_km
    );

    let speed_mi = Milesh { value: 21 };
    let distance_in_miles = speed_mi.distance_in_three_hour();
    println!(
        "At :{:?} you travel :{:?} in 3 hours",
        speed_mi, distance_in_miles
    );
}

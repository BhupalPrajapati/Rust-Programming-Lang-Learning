// here we can define two struct for the squire and rectangle

// struct Drwaing{
//     line_width : u8,
//     color : String,
// }

// use std::task::ready;

struct Square {
    side: f32,
    line_width: u8,
    color: String,
}
struct Rectangle {
    lenth: f32,
    width: f32,
    line_width: u8,
    color: String,
}
// here we implement the both struct by define the method inside the impl block
/*
impl Square {
    /// Returns the calculate area of this [`Square`].
    fn calculate_area(&self){
        println!("The area of square is {}",self.side*self.side );
    }
}
impl Rectangle {
    /// Returns the area of this [`Rectangle`].
    fn area(&self) ->f32{
        self.lenght*self.width

    }

}
*/

// Here we are define the Super trait
trait Draw {
    fn draw_object(&self);
}

// we going to define the trait in this langauge

trait Shape: Draw + OtherTrait + SomeTrait {
    fn area(&self) -> f32;
    // define the trait for the perimeter
    fn perimeter(&self) -> f32 {
        println!("Perimeter is implemented for Square, returing the just dummy value");
        0.0
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.lenght * self.width;
        println!("Area of Rectangle is :{}", area_of_rect);
        area_of_rect
    }
    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.lenght * self.width);
        println!("Perimeter of Rectangle is :{}", perimeter_of_rect);
        perimeter_of_rect
    }
}
// imple,emtaion of Square
impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_squre = self.side * self.side;
        println!("Area of Squre is :{}", area_of_squre);
        area_of_squre
    }
    // fn perimeter(&self)->f32 {
    //     let perimeter_of_square = 2.0*(self.side*self.side);
    //     println!("Perimeter of Square is:{}",perimeter_of_square);
    //     perimeter_of_square
    // }
}

// lets implementation of draw trait for the rectangle

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
    }
}
// letd implementation of Shape trait for the super
// #[warn(bare_trait_objects)]
impl Draw for Shape {
    fn draw_object(&self) {
        println!("Drawing Shape");
    }
}

// Marker trait

trait OtherTrait {}
// impl OtherTrait for  Shape {}
impl OtherTrait for Rectangle {}
impl OtherTrait for Shape {}

trait SomeTrait {}
impl SomeTrait for Shape {}
impl SomeTrait for Rectangle {}

// Traits Bounds

fn shape_properties_static<T>(object: T)
where
    T: Shape,
{
    object.area();
    object.perimeter();
}

// Dynmic dispatcher(specific implemetaion is not generated at a compile time)
fn shape_properties_dynmic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}

// lets deifne fn for each specified

fn shape_properties_sh(object: Shape) {
    object.area();
    object.perimeter();
}
fn shape_properties_rec(object: Rectangle) {
    object.area();
    object.perimeter();
}
// retunr types of triats

fn returs_shape(dimension: Vec<i32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        let sq = Square {
            side: dimension[0],
            line_width: 1,
            color: String::from("BLack"),
        };
        Box::new(sq)
    }else {
        let rect = Rectangle{
            lenth:dimension[0],
            width:dimension[0],
            line_width:1,
            color:String::from("Red"),
        };
        Box::new(rect)
    }
}
// Now define the struct of the circle

struct Circle {
    radius: f32,
}

fn main() {
    let r = Rectangle {
        lenth: 12.1,
        width: 3.2,

        line_width: 3,
        color: String::from("Red"),
    };
    //  println!("Area of Rectangle :{}",r.area() );

    let s = Square {
        side: 3.2,
        line_width: 2,
        color: String::from("Blue"),
    };
    //  println!("Area of Squre:{}",s.area() );

    let c = Circle { radius: 1.2 };
    r.area();
    s.area();
    r.perimeter();
    s.perimeter();
    // shape_properties(r);
    // shape_properties_rec(s);
    // shape_properties_sh(c);

    shape_properties_dynmic(Box::new(r));
    shape_properties_dynmic(Box::new(s));
}

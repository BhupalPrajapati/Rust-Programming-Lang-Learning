trait Drawable {
    type Sometype; // i have defined the associated type of trait
    fn draw(&self);
}

struct Circle {
    x: i64,
    y: i64,
}

struct Square {
    x: i64,
    y: i64,
}

impl Drawable for Circle {
    type Sometype = i64;
    fn draw(&self) {
        println!("Drawing Circle at ({}, {})", self.x, self.y);
    }
}

impl Drawable for Square {
    type Sometype = i64;
    fn draw(&self) {
        println!("Drawing Square at ({}, {})", self.x, self.y);
    }
}

// fn draw(obj:&dyn Drawable) {   // this is gives the error because it is demanding the associated type
//                                // it's value is not contains in the v table , in the v table contains the pointet , so that it's is not work trait obj directly
//     obj.draw();
// }

fn draw(obj: &dyn Drawable<Sometype = i64>) {
    // this is compiled because it is demanding the associated type
    obj.draw();
}

fn main() {
    let circle = Circle { x: 5, y: 5 };
    let square = Square { x: 10, y: 10 };
    draw(&circle);
    draw(&square);
    println!("Circle's area: {}", circle.x * circle.y);
    println!("Square's area: {}", square.x * square.y);
    // println!("Circle's area: {}", circle.draw()); // this gives the error because draw method return nothing
}

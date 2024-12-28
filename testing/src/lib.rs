#[warn(dead_code)]
mod shape{
    #[warn(dead_code)]
    pub struct Circle {
        radius: f64,
    }
    impl Circle {
        pub fn new(radius:f64)->Circle{
            println!("Congratulations all test are passed");    
            Circle{radius}
        }

        pub fn new1(radius:f64)->Circle{
            match radius{
                // r if r>0.0=>Circle{radius:r},

                -10.0..=0.0=>panic!("is between -10 and 0"),
                ..=-10.0=>panic!("radius must be positive"),
                _=>Circle{radius}
                
            }
        }
        pub fn contains(&self,other:&Circle)->bool{
            self.radius>other.radius
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]

        fn larger_circle_contains_smaller_circle(){
          let s = shape::Circle::new(2.0);
          let l = shape::Circle::new(3.0);
          assert!(l.contains(&s),"Customer failure message ");

        }
    #[test]
    fn smaller_circle_contains_larger_circle(){
        let s = shape::Circle::new(2.0);
        let l = shape::Circle::new(3.0);
        assert!(!s.contains(&l));
    }

    // for the panic , if the any value is goes into the negative value then the test is panic and show in the console

    #[test]
    #[should_panic(expected="is lesser than -10")]
    fn negative_radius(){
        let _s = shape::Circle::new1(-8.0);
    }



    // if you want to ingone any test then you need annonate the test with ignore

    // if you want to test only this test(ignopre test), then you need pass in the terminal cargo test --lib -- --show-output

    #[test]
    #[ignore]
    fn huge_test(){
        //
    }

}
// we have written the code for the library, now going to used the same 
// so we need to inmport the first these files here

 use testing::shape::Circle;

#[test]

fn test_create_circle_with_positive_radius() {
    let c = Circle::new(2.0);
    assert_eq!(c.contains(&Circle::new(2.0)), true);
}

#[test]
fn test_create_cricle_with_zero_radius() {
    let c = Circle::new(0.0);
    assert_eq!(c.contains(&Circle::new(0.0)), false);
}

#[test]
#[should_panic(expected = "radius must be positive")]
fn test_create_circle_with_negative_radius() {
    let _c = Circle::new1(-10.0);
}

#[test]
fn test_smaller_does_not_contain_larger_circle(){
    let s = Circle::new(2.0);
    let l = Circle::new(3.0);
    assert_eq!(!s.contains(&l), true);
}

// if you only obtained the report of the integration test, we can use the test flag in the command line
// for eg.cargo test --test order
       
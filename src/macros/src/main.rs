macro_rules! our_macro {
    () => { 2+1

    };

    (hAJBQJBDS432)=>{             // the left side must contanis the any matching rules
        println!("Hello, world!");// nd right side must be contains the vaild rust somethings
    };

    // oen the part of the macros is called captures, here we define the capturess(the captures uses $ sign)
    ($e1:expr, $e2:expr) => {$e1+$e2   // the semicolon is optional in macros, if i remove the last line of semicolon, then the compiler is happy
    }
}

fn main() {
    our_macro!();
    println!("{}", our_macro!()); // remember that is not a function call

    // in macros rule left side applied pattern must be when we call instance of the macro
    our_macro!(hAJBQJBDS432);

    println!("{}", our_macro!(2, 2));
}

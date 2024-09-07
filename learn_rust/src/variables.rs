fn main() {
    // Mutability
    let mut x = 1;
    x += 2;

    // shadowing
    let y = 6;
    let y = 7;

    // const
    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;

    scope_example();
}

// Scope of variable is defined by the block of code in which it is declared
fn scope_example() {
    // let x = 10;

    // {
    //     let y = 5;
    //     println!("The value of x is {} and the value of y is {}", x, y)
    // }

    // This won't work because is outside our scope
    // println!("The value of x is {} and the value of y is {}", x, y)

    // ---------

    let x = 10;
    let y = 5;

    {
        // works
        println!("The value of x is {} and the value of y is {}", x, y)
    }

    // works
    println!("The value of x is {} and the value of y is {}", x, y)
}

fn shadowing_example() {
    let x = 5;

    let x = x + 1;

    {
        // shadowing is limited to the scope in which it is defined
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // X = 6

    // shadowing difference from mutability
    // it lets you change the type of a variable
    let spaces = "   ";
    let spaces = spaces.len();

    // after redeclaring a variable, it is no longer mutable
    // spaces = spaces.len(); // error
}

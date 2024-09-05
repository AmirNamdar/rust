fn main() {
    // Initialization
    // let a: i32; // Uninitialized but used, ERROR
    // let _b: i32; // Uninitialized but unused, WARNING, we fix the warning by prefixing the var with "_"
    
    // assert_eq!(a, 5)

    
    // ----------------------------------------------------------------------

    // Mutability
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success");


    // ----------------------------------------------------------------------

    scope_example();

    
}


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

// Scope of variable is defined by the block of code in which it is declared

// Shadowing allows a variable to be redeclared in the same scope
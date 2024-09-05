fn main() {
    let x: i32 = 5;

    {
        // this declaration scope is the inner scope of the curly braces
        // it does not affect the outer scope
        let x = 12;
        assert_eq!(x, 12); // correct
    }

    assert_eq!(x, 5); // correct
    let x: i32 = 42; // this declaration shadows the one in line 2

    println!("{}", x); // Prints 42
}

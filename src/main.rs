fn main() {
    // --------- Variables ---------
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // --------- Constants ---------
    // Constants are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the constant: {THREE_HOURS_IN_SECONDS}");

    // --------- Shadowing ---------
    // The ability to declare a new variable with the same
    // name as a previous variable. We say that the first variable
    // is shadowed by the second
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12
    }

    println!("The value of y is : {y}"); // 6
}

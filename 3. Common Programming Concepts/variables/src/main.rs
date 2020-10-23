fn main() {
    //Variables and Mutability
    let x = 5;
    println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Differences Between Variables and Constants
    const MAX_POINTS: u32 = 100_000;

    //Shadowing

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

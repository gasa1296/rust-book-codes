fn main() {
    println!("Hello, world!");

    another_function1();
    another_function2(5, 6);

    let y = 6;

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    
    let x = five();

    println!("The value of x is: {}", x);
}

fn another_function1() {
    println!("Another function.");
}
//Function Parameters
fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

//Functions with Return Values
fn five() -> i32 {
    5
}
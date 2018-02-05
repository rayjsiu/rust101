fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;    // Destructuring the tuple to individual variables

    println!("The value of y is: {}", y);
}

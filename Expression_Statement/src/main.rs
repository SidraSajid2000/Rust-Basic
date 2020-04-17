fn main() {
    let mut x  = 5;

    let y = {
        x = x +  3;
        println!("The value of inside x is: {}", x);
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of outside x is: {}", x);
}

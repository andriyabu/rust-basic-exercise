fn main() {
    let x = 5;
    println!("x : {}", x);
    
    // shadowing technic to isolate variable in a block of code.
    let x = x + 1;
    println!("x : {}", x);
}

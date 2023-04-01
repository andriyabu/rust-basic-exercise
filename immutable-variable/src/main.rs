fn main() {
    let message_number = 1;
    let message1 = "message";
    println!("{} : {}",message1,message_number);
    message_number = 2;
    let message2 = "message 2";
    // will throw erro becouse by default variable is set to immutable, it's mean the value of variable cannot be change.
    println!("{} : {}", message2, message_number);
}

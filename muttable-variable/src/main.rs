fn main() {
    // keyword "mut" after "let" indicated that the variable is muttable variable
    let mut message_number = 1;
    let message = "message";
    println!("{} : {}", message, message_number);

    // now the value of variable message_number can be change by other value with the same type of variable
    message_number = 2;
    println!("{} : {}", message, message_number);
}

fn main() {
    let (var1, mut var2, var3) : (i8,i8,i8) = (1,2,3);
    println!("var1 : {}, var2 : {}, var3 : {}", var1,var2,var3);
    var2 = 5;
    println!("var2 : {1}, var1 : {0}, var3 : {2}", var1,var2,var3);
}

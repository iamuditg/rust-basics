/*
 variables - let
*/
fn variables() {
    let mut  x: u8 = 255;
    println!("x is {}",x);
    // x is immutable i.e variable are immutable in rust
    // for mutable define variable with mut keyword
    // x = 20
    x = 30;
    println!("x is {}",x);
}
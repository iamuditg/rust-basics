fn modules() {
    // crates
    let number = rand::random::<f64>();
    println!("number is {}",number);

    // standard input
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}",buffer);

    // parse strings
    let number : i32 = buffer.trim().parse().unwrap();
    println!("number is +1 {}",number+1);
}

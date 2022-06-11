fn input_and_output() {
    // reading from files
    println!("Read files...");
    let contents = fs::read_to_string("./src/planets.txt").unwrap();
    println!("contents is {}",contents);
    // writing to files
    let mut speech = String::new();
    speech.push_str("We choose to go to the moon in this decade\n");
    speech.push_str("and do the other things,\n");
    fs::write("./src/planets.txt",speech);

    //let mut file = fs::OpenOptions::new().append(true).open("./src/planets.txt").unwrap();
    //fs::write(b"\npluto");

    // command line arguments
    if env::args().len() <= 2 {
        println!("Program requires as least 2 arguments");
        return;
    }
    for (index,argument) in env::args().enumerate() {
        println!("argument {} is {}",index,argument);
    }
    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
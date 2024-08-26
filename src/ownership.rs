

pub fn ownership_concept() {
   //string_one_ex();
    // string_two_ex();
    slice_ex_one();
}

fn slice_ex_one() {
    let mut mu_string = String::from("hello world");
    // 'first_word' works on slices of ;string;s whether partial
    // or whole
    let word = first_word(&mu_string[0..6]);
    let word = first_word(&mu_string[..]);
    //s.clear();
    let word = first_word(&mu_string);

    println!("{word}")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn string_two_ex() {
    let s1 = gives_ownership(); // gives ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(&s2); // s2 is moved into takes and gives back which also moves its return value into s3
    println!("s3: {s3} {s2}");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    let mut refr = String::from("hello");
    let  a1 = &refr;
    let a2 = &refr;
    println!("{a1},{a2}");

    let r3 = &mut refr;
    println!("{r3}");
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world");
}

fn takes_and_gives_back(a_string: &String) -> &String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn string_one_ex() {
    // string ownership
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    // string clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into function...
    let x = 5; // xcomes into scope
    makes_copy(x); // x would into function, but i32 is Copy , so its okay to still use x afterward
    // Here x goes out of scope , then s. However s's value was movedd nothing special happens.
}

fn makes_copy(p0: i32) {
   println!("{p0}");
    // Here, some_integer goes out of scope. Nothing special happens.
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
    // Here some_string goes out of scope and drop is called. the backing memory is freed.
}
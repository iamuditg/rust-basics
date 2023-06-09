pub fn tuples() {
    let a:(i32,bool,f64) = (220,true,8.5);
    println!("{:?}",a.0);
}

pub fn array() {
    // let a:[i32;5]=[1,2,3,4,5];
    let mut a:[i32;5]=[0;5];
    a[3] = 1;
    println!("{:?}",a);
    let b = [3;5];
    println!("b: {:?}",b)
}
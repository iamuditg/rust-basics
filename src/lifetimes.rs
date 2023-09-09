
/*
- Lifetimes
*/
pub fn lifetimes_ex() {
    // let i: &i32;
    // {
    //     let j: i32 = 5;
    //     i = &j;
    // }
    // println!("{}",i);

    let some_int = 10;
    let additional_int = some_fn(&some_int);
    println!("{}",additional_int);

    let int1 = 5;
    let int2 = 10;

    let result = greater(&int1,&int2);
    println!("{}",result);

    let s_1 = "Hello";
    let v;
    {
        let s_2 = String::from("World");
        v = some_fn_ex(s_1,s_2.as_str());
    }
    println!("{}",v);

    let int1_ex = 15;
    {
        let int2_ex = 110;

        let result = greater_ex(&int1_ex,&int2_ex);
        println!("{}",result);
    }


}


fn greater_ex<'a,'b>(i: &'a i32,j: &'a i32) ->&'a i32 {
    if i > j {
        i
    }else {
        j
    }
}

fn greater(i: &i32,j: &i32) ->i32 {
    if i > j {
        *i
    }else {
        *j
    }
}

fn some_fn_ex<'a,'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
    first_str
}


// fn greater(i: &i32, j: &i32) -> &i32 {
//     if i > j {
//         i
//     }else {
//         j
//     }
// }

fn some_fn(i: &i32) -> &i32 {
    &i
}
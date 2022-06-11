fn flow_control() {
    let x = 3;
    let y = 5;
    // conditional
    if x > y {
        println!("x is greater than y");
    }else {
        println!("x is not greater than y");
    }

    // loops
    let mut count = 0;
    loop {
        if count == 10 {
            break;
        }
        count += 1;
        println!("count is {}",count)
    };

    // while loops
    count = 0;
    let letters = ['a','b','c'];
    while count < letters.len() {
        println!("count is {}",letters[count]);
        count += 1;
    }

    // for loops
    let message = ['a','b','c','d','e'];
    for (index,&item) in message.iter().enumerate() {
        println!("item {} is {}",index,item);
        if item == 'c'{
            break;
        }
    }
    let mut matrix = [[1,2,3],[4,5,6],[7,8,9]];
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t",num);
        }
        println!();
    }
}
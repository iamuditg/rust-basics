
#[derive(Debug)]
struct Rectangle<T,U> {
    width: T,
    height: U
}

impl<T,U> Rectangle<T,U>  {
    fn get_width(&self) -> &T {
        &self.width
    }
}

fn get_biggest<T: PartialOrd>(a: T,b: T) -> T {
    if a > b {
        a
    }else {
        b
    }
}

fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>,b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}

fn generic_types() {
    let rec = Rectangle{ width: 1, height: 3u16 };
    print!("rect is {:?}",rec);
    print!("width is {}",rec.get_width());
    print!("biggest number is {}",get_biggest(3,4));

    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one,two),3);
    print!("Test Passed");
}

trait Summary {
    //fn summarize(&self) -> String
}
pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})",self.headline,self.author,self.location)
//     }
// }
use std::io;

pub fn variable_test(){
    // basic_test();
    // variable_covertest();
    // tuple_test();
    // array_test();
    index_test();
}
pub fn basic_test(){
    // let  x = 5; 默认是不可变的
    let mut x = 5;
    println!("The value of x is: {}", x);
    x =6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
    // THREE_HOURS_IN_SECONDS = 60 * 60 * 4; // 这行会报错，因为常量是不可变的
}

pub fn variable_covertest(){
    let x = 5;

    let x = x+1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);
}

pub fn tuple_test(){
    let tuple:(i32,f64,u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("The value of x is: {}, y is: {}, z is: {}", x, y, z);
    let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let one = tuple.2;
    println!("The value of five_hundred is: {}, six_point_four is: {}, one is: {}", five_hundred, six_point_four, one);
}

pub fn array_test(){
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
    println!("The first element of a is: {}", a[0]);
    println!("The second element of a is: {}", a[1]);
    println!("The third element of a is: {}", a[2]);
    println!("The fourth element of a is: {}", a[3]);
    println!("The fifth element of a is: {}", a[4]);
}

fn index_test(){
    let a = [1,2,3,4,5];
    println!("enter a index!");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index : usize = index.trim().parse().expect("Please type a number!");

    let element = a[index as usize];
    println!("The value of element  at index {} is: {}",index,element);
}
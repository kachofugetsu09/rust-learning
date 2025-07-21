pub fn control_flow_test() {
    // if_else();
    // else_if_test();
    // condition_test();
    // loop_test();
    // return_from_loop();
    // while_test();
    // for_array();
    simple_for();
}

pub fn if_else(){
    let number = 7;
    if number <5 {
        println!("is smaller than 5");
    }
    else{
        println!("is bigger than 5");
    }
}

pub fn else_if_test(){
    let n = 6;
    if n%4 ==0{
        println!("n is divisible by 4");
    }
    if n == 6{
        println!("n is 6");
    }
        //else if排他，如果想要同时判断if和else if的条件都满足，可以使用多个if语句
    else if n%3 == 0{
        println!("n is divisible by 3");
    }
    else{
        println!("nice try");
    }
}

pub fn condition_test(){
    let condition = true;
    let number = if condition{3} else {4};
    println!("The value of number is: {}", number);
}

fn loop_test (){
    let mut count = 0;
    'counting_up:loop{
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -=1;
        }
        count+=1;
    }
    println!("count is {}", count);
}

fn return_from_loop(){
    let mut counter = 0;
    let result = loop {
        counter+=1;

        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn while_test(){
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -=1;
    }
    println!("LIFTOFF!!!");
}

fn for_array(){
    let a = [10,20,30,40,50];
    for element in a.iter(){
        println!("The value is: {}", element);
    }
    //不包含4 创建一个序列 然后逆向迭代
    for number in (1..4).rev(){
        println!("{}!", number);
    }

    let mut i = 0;
    while i<5 {
        println!("the value of i is: {}", a[i]);
        i+=1;
    }
}

fn simple_for(){
    let a = [10,20,30,40,50];

    for element in a{
        println!("the value is: {}", element);
    }
}
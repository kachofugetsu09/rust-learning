pub fn ownership_test(){
    // owner_move_test();
    // clone_test();
    // stack_test();
    borrowing_test();
}

pub fn owner_move_test(){
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{},world!",s1); // 这行会报错，因为s1的所有权已经转移给了s2
    println!("{},world!",s2); // 这行可以正常打印，因为s2现在拥有字符串的所有权
}

fn clone_test(){
    let s1 = String::from("huashen");
    let s2 = s1.clone();
    // 这里s1和s2都拥有字符串的所有权
    println!("s1 = {}, s2 = {}",s1, s2);
}

fn stack_test(){
    let x = 5;
    let y = x;
    // 这里x和y都是整数类型，存储在栈上,所以x还存在。
    println!("x = {}, y = {}", x, y);
}

fn test_takes_ownership(){
    let s = String::from("huashen");

    takes_ownership(s);

    // let y =s; // 这行会报错，因为s的所有权已经转移给了takes_ownership函数

    let x = 5;
    makes_copy(x);
    let y = x; // 这行不会报错，因为i32是一个Copy类型，x的值被复制给了y
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn test_giving_ownership(){
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回some_string的所有权
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("I got a string {}",a_string);
    a_string // 返回a_string的所有权
}

fn reference_test(){
    let s1 = String::from("hello");

    //这里没有转移s1的所有权，创建了一个&s1的引用，
    // 这个引用允许你使用上面的值，但是不获取所有权。
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s 是对 String 的引用
    s.len()
}

fn borrowing_test(){
    // let s = String::from("hello");
    let mut s = String::from("hello");
    change(& mut s);
    println!("{}",s);
}

fn change(some_string: &mut String){
    // 这里不能修改some_string，因为它是一个不可变引用
    // some_string.push_str(",world!");
    some_string.push_str(", world");
}
fn only_one_test(){
    let mut s = String::from("hello");

    let r1 = &mut s;
    //这里会报错。因为同一时间只能有一个可变引用。
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

fn first_word(s : &String) -> usize{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        //如果找到空格，这里需要b,因为迭代的是byte
        if item == b' '{
            return i;
        }
    }
    s.len()
}


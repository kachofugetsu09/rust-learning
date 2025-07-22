pub fn main(){
    life_test();
}

fn life_test(){
    let r;
    {
        let x = 5;
        r = &x; // r的生命周期是x的生命周期
        println!("r: {}", r);
    }
    //rust不允许自己引用的对象的生命周期比自己短
     // println!("r: {}", r); // 这行会报错，因为x的生命周期结束后，r指向的内存不再有效
}

fn logest <'a>(x: &'a str, y:&'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn test() {
    // let result;
    { // 内部作用域
        let s1 = String::from("hello"); // s1 在这里创建
        let s2 = String::from("world!"); // s2 在这里创建

        // 调用 logest，s1 和 s2 的生命周期都只在这个内部作用域
        //在这里会出现一个报错
        // result = logest(&s1, &s2);
        // 在这里，编译器推断出 logest 返回的引用生命周期 `'a` 绑定到 s1 和 s2 的生命周期。
        // 这个生命周期在当前内部作用域结束时就失效了。
    } // s1 和 s2 在这里被销毁，它们占用的内存被释放。
    // 但是 result 仍然试图引用已经被销毁的内存！
    // println!("{}", result); // 编译错误！因为 result 的生命周期比它引用的数据长。
}

fn lon_test(){
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = logest(string1.as_str(), string2.as_str());
        println!("The largest string is {}", result);
    }
}

struct ImportantExcerpt<'a>{
    part : &'a str,
}

fn importantExcerpt_test(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("Important excerpt: {}", i.part);

    {
        let s: &'static str = "I have a static lifetime.";
    }
    // println!("Static string: {}", s);
}

fn print_greeting(name: &str) { // name 是一个引用
    println!("Hello, {}!", name);
}

fn greeting_test() {
    let name = String::from("Alice");
    print_greeting(&name); // 传递一个引用

}

fn find_prefix_in_text<'a,'b> (prefix: &'a str, text: &'b str) -> Option<&'a str> {
    if text.starts_with(prefix){
        Some(prefix)
    } else {
        None
    }
}

fn test_prefix(){
    let app_name = String::from("Rust");
    let full_log_message = String::from("Rust is a systems programming language.");

    let found = find_prefix_in_text(&app_name, &full_log_message);

    if let Some(prefix) = found{
        print_greeting("Found prefix:");
    }
    else{
        print_greeting("Prefix not found.");
    }


    println!("--- Separator ---");

    let long_lived_prefix = String::from("Debug"); // 这个前缀在外部作用域
    let result_of_search;

    { // 内部作用域
        let short_lived_log = String::from("Debug: Something happened."); // 这个日志在内部作用域
        // 在这里，'a 绑定到 long_lived_prefix 的生命周期
        // 'b 绑定到 short_lived_log 的生命周期
        result_of_search = find_prefix_in_text(&long_lived_prefix, &short_lived_log);

        if let Some(s) = &result_of_search { // 这里不用&resultofsearch就会move到s了
            println!("Inside scope, found: {}", s);
        }
    } // short_lived_log 在这里被销毁！

    // println!("Outside scope, found: {}", result_of_search.unwrap());
    // 如果尝试在这里使用 result_of_search.unwrap()，将导致编译错误！
    // 因为 result_of_search 返回的引用生命周期 ('b) 绑定到了 short_lived_log，
    // 而 short_lived_log 在内部作用域结束后就失效了。
    // 这再次证明了生命周期参数的实际作用。

    // 为了避免编译错误，我们只能在 short_lived_log 有效的范围内使用 result_of_search
    println!("Operation completed.");

}
use std::collections::*;
pub fn main(){
    vector_test();
    hash_map_test();
}

fn vector_test(){
    let  mut v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);
    //既可以用索引也可以用get方法来获取元素
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let first = &v[0];
    //当获取了一个vector当中的不可变引用后，不能再对这个vector进行修改操作。
    // v.push(6);
    println!("The first element is {}", first);

    for i in &v{
        println!("Element: {}", i);
    }

    for i in &mut v{
        *i += 10; // 使用解引用来修改元素
        println!("Modified Element: {}", i);
    }

    //可以通过枚举类来向一个vector中存储不同类型的数据。
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![SpreadsheetCell::Int(1), SpreadsheetCell::Text(String::from("blue"))];

    let data = "inital data";
    let mut s = data.to_string();
    let _s1 = "initial data".to_string();
    s.push_str(" and more data");
    println!("s: {}", s);

    let _s2 = String::from("Hello, world!");
    // let s3 = s+ &s2;
    // println!(s);
}

fn hash_map_test(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _socre:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    let _s = scores.get(&String::from("Blue"));
    let yell = String::from("Yellow");
    let _res = scores.get(&yell);

    for(k,v) in &scores{
        println!("{}: {}", k, v);
    }

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(30);
}


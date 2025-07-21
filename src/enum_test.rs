enum ipAddrKind{
    v4,
    v6,
}
pub fn main(){
    // do_match()
    if_let_test();
}

fn if_let_test(){
    let x = Some(0u8);
    if let Some(3) = x{
        println!("three");
    }
}

fn do_match(){
    let x:i8 = 5;
    //由于y是Option,所以编译器不同意你直接进行加法运算。
    // 我们使用match来处理Option类型。
    let y:Option<i8> = Some(5);

    let sum = match y {
        Some(n) =>n+x,
        None =>
            {
                println!("No value found!");
                x
            }
    };
    println!("Sum is {}", sum);

    let y_none: Option<i8> = None;
    let sum_none = match y_none {
        Some(val) => x + val,
        None => {
            println!("y_none 没有值，无法进行加法运算。使用默认值 0。");
            0
        },
    };
    println!("sum_none = {}", sum_none);

}
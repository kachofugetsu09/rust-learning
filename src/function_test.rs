pub fn function_test(){
    println!("Hello, world!");
    another_function(3,'a');
    statement_expression_test();
}
fn another_function(x:i32,unit_label:char){
    println!("Another function.");
    println!("The value of x is: {},unitlabel is {}",x,unit_label);
}
fn statement_expression_test(){
    let x = 1;
    let y = {
        let x = x+2;
        x+5
    };
    println!("The value of x is: {} and y is: {}",x,y);
}
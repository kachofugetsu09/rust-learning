pub fn main(){
    let mut a = vec![1,2,3,4,5];
    let mut b = Vec::new();
    for i in a.into_iter(){
        if i%2==0{
            b.push(i);
            println!("Even number: {}", i);
        }
    }
    // println!("Remaining elements in a: {:?}", a); //这一行直接报错了，因为在使用iterinto之后整个vector都没了。
    //反观用drain的他只会从原本的vector中移除元素，而不是将整个vector消耗掉。
    println!("Elements moved to b: {:?}", b);
}
// pub fn main(){
//     let mut a = vec![1,2,3,4,5];
//     let mut b = Vec::new();
//     for i in a.drain(..){
//         if i%2==0{
//             b.push(i);
//             println!("Even number: {}", i);
//         }
//     }
//     println!("Remaining elements in a: {:?}", a);
//     println!("Elements moved to b: {:?}", b);
// }





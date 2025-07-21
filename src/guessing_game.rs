use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn play_game(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..200);

    // println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        //readline的全部工作是将用户在标准输入中输入的任何内容中追加到一个字符串中。
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        //为什么必须要添加trim是因为用户输入enter后会在字符串中添加换行符\n
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("别输入无效的类型");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
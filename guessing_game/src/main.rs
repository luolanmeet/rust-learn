use std::cmp::Ordering;
use std::io;
use std::io::Stdin;
use rand::Rng;

fn main() {

    println!("猜数字游戏");

    // 生成随机数
    let secret_number : u32 = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {}", secret_number);
    loop {

        println!("请输入一个数字");
        // rust 变量默认不可变，加 mut 关键字可以让变量变成 可变变量
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // 用相同变量名，隐藏写法，一般用于类型转换
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // println!("You guessed: {}", guess);

        // match 有点像 switch
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}

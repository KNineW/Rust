use std::io; //prelude
use rand::{Rng}; //trait
use std::cmp::Ordering; //枚举

fn main() {
    println!("猜数!");

    let secret_number= rand::thread_rng().gen_range(1..=100); //生成一个随机数

    //println!("秘密数字是: {}", secret_number); //调试用，正式发布时可以注释掉
    
    loop{
        println!("猜测一个数");
//let foo=1;
//let bar=foo;    默认immutable不可变，加入mut 则是可变的
        let mut guess = String::new(); //声明一个变量

        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字!");
                continue;
            }
        }; //shadow
        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less=> println!("太小了!"),
            Ordering::Greater=> println!("太大了!"),
            Ordering::Equal=> {
                println!("你赢了!");
                break;
             }
         }
    
     }
 }

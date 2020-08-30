extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // 猜数游戏
    let mut str = String::new();
    // 第三放包 引用
    let rand_number = rand::thread_rng().gen_range(1, 1000);

    loop {

        println!("please input guess number");
        io::stdin().read_line(&mut str).expect("failed to read line");

        let num: u32 = match str.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("you input {} ", str);
        // switch case
        match num.cmp(&rand_number) {
            Ordering::Equal => {
                println!("恭喜你,猜对了!");
                break;
            }
            Ordering::Less => println!("猜错了,太小了~"),
            Ordering::Greater => println!("猜错了,太大了!!"),
        }
        str.clear();
    }

}

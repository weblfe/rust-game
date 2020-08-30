extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    game();
}

// 游戏逻辑
fn game() {
    // 猜数游戏
    let mut count: u32 = 0;
    let mut str = String::new();

    // 随机数 第三放包 引用
    let rand_number = rand::thread_rng().gen_range(1, 1000);

    loop {
        str.clear();
        count = count + 1;
        println!("请输入你猜想的数字: ");
        match io::stdin().read_line(&mut str) {
            Ok(n) =>n,
            Err(err) => {
                println!("输入有误, {}", err);
                continue;
            }
        };

        let num: u32 = match str.trim().parse() {
            Ok(n) => {
                println!("你输入的数字是: {}",n);
                n
            },
            Err(_) => continue,
        };

        // switch case
        match num.cmp(&rand_number) {
            Ordering::Equal => {
                println!();
                println!("恭喜你,猜对了!");
                println!("总猜错了 {} 次", count - 1);
                break;
            }
            Ordering::Less => println!("猜错了,太小了~"),
            Ordering::Greater => println!("猜错了,太大了!!"),
        }
    }
}
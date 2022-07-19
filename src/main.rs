use std::cmp::Ordering;
use std::io;
//prelude
use rand::Rng;

fn main() {
    // 变量不可变性, 需要加上mut才能变化
    println!("Hello, World");
    let mut x = 5;
    println!("x:{}", x);
    x = 6;
    println!("x:{}", x);

    // shadowing隐藏同名变量
    let y = 5;
    let y = y + 1;
    println!("the value of y is {}",y);

    let space = "    ";
    let space = space.len();
    println!("space len = {}",space);
}

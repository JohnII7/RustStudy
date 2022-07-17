use std::io; //prelude
use rand::Rng; //trait
fn main() {
    println!("猜个数字");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("神秘数字是{}",secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法执行");
    println!("你猜的是: {}",guess);
}

use std::io;

fn main() {
    println!("猜个数字");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法执行");
    println!("你猜的是: {}",guess);
}

use std::cmp::Ordering;
use std::io;
//prelude
use rand::Rng;

//trait
fn main() {
    println!("猜个数字");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是{}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法执行");

    // shadow
    let guess:u32 = guess.trim().parse().expect("Please type a number");

    println!("你猜的是: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less =>println!("Too small !"),
        Ordering::Greater=>println!("Too big!"),
        Ordering::Equal=>println!("You win"),
    }
}

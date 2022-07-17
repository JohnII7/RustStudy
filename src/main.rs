use std::cmp::Ordering;
use std::io;
//prelude
use rand::Rng;

//trait
fn main() {
    println!("猜个数字");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法执行");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("你猜的是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

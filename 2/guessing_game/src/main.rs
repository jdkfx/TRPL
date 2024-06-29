use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("数の推測");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    // println!("秘密の数字: {}", secret_number);

    loop {
        println!("数字を入力");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("次のように予想しました: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("その数より大きいです: {}", guess),
            Ordering::Greater => println!("その数より小さいです: {}", guess),
            Ordering::Equal => {
                println!("等しい");
                break;
            }
        }
    }
}

use std::io;

fn main() {
    // (0°C×9÷5) + 32 ＝ 32°F
    println!("摂氏（°C）を入力");

    let mut c = String::new();
    
    io::stdin()
        .read_line(&mut c)
        .expect("読み込みに失敗しました");

    let c: usize = c
        .trim()
        .parse()
        .expect("入力された値は数字ではありません");

    let f = (c * 9 / 5) + 32;

    println!("摂氏（°C）: {}, 華氏（°F）: {}", c, f);
}

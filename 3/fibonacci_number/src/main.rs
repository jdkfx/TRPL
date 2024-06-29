use std::io;

fn main() {
    println!("nの値を入力");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("読み込みに失敗しました");

    let n: usize = n
        .trim()
        .parse()
        .expect("入力された値は数字ではありません");

    let result = fibonacci(n);
    println!("フィボナッチ数列のn番目の値は{}です", result);
}

fn fibonacci(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
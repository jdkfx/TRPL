use std::io;

fn main() {
    let a = [1, 3, 5, 7, 9];

    println!("配列の何番目の要素にアクセスしますか？");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("読み込みに失敗しました");

    let index: usize = index
        .trim()
        .parse()
        .expect("入力された値は数字ではありません");

    let element = a[index];

    println!("{}番目の要素の値は{}です",
        index, element
    );
}

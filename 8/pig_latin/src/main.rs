// 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。
// 各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。
// ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。
// UTF-8エンコードに関する詳細を心に留めておいてください！

use std::io;

fn main() {
    println!("アルファベット文字列を入力してください");
    let mut text = String::new();

    io::stdin()
            .read_line(&mut text)
            .expect("読み込みに失敗しました");
    
    if !text.is_ascii() {
        println!("条件にあった文字列を入力してください");
        return
    }
    
    let mut buffer = text.trim().to_string();
    
    let first_char = buffer.chars().next().unwrap();
    
    if check_vowel(first_char) {
        buffer.push_str("-hay");
        println!("{}", buffer);
    } else {
        buffer.remove(0);
        let ans = format!("{}-{}ay", buffer, first_char);
        println!("{}", ans);
    }
}

fn check_vowel(text: char) -> bool {
    "aiueo".contains(text)
}
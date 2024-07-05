// 整数のリストが与えられるので、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、
// mode(最も頻繁に出現する値:ハッシュマップがここでは有効活用する)を返してください。

use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1, 1, 2, 6, 5, 7, 7, 6, 4, 1, 1, 2, 4, 2, 1];
    let length = numbers.len();

    let mut sum = 0;
    for i in &numbers {
        sum += *i;
    }
    println!("合計値: {}", sum);

    let ave = sum / length;
    println!("平均値: {}", ave);

    numbers.sort();
    let i = length / 2;
    if length % 2 == 0 {
        let median = (numbers[i - 1] + numbers[i]) / 2;
        println!("中央値: {:?}", median);
    } else {
        let median = numbers[i];
        println!("中央値: {:?}", median);
    }

    let mut mode = HashMap::new();
    for number in numbers {
        let count = mode.entry(number).or_insert(0);
        *count += 1;
    }
    let max_count = mode.values().cloned().max().unwrap_or(0);
    println!("最頻値: {:?}", mode[&max_count]);

    println!("（小数点以下切り捨て）")
}

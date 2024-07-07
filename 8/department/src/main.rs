// ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。 
// 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。
// それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。
// Qiita記事を参考に実装

use std::collections::HashMap;
use std::io;

fn main() {
    println!("雇用者リスト");
    
    let mut employee_list: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("コマンドを入力してください: (例)Add Sally to Engineering");

        let mut input_command = String::new();
        io::stdin()
            .read_line(&mut input_command)
            .expect("読み込みに失敗しました");

        let v: Vec<&str> = input_command
            .trim()
            .split_whitespace()
            .collect();
        let command = v[0];

        match command {
            "Add" => {
                if v.len() != 4 {
                    println!("Addコマンドは引数が三つ必要です");
                    continue;
                }
                let employee_name = v[1].to_string();
                let department_name = v[3].to_string();
                let v = employee_list.entry(department_name.clone()).or_insert(vec![]);
                match v.iter().position(|x| *x == employee_name) {
                    Some(_) => {
                        println!("{}は既に登録されています", employee_name);
                    },
                    None => {
                        v.push(employee_name.clone());
                        println!("{}を{}に追加しました", employee_name, department_name);
                    }
                }
            },

            "Quit" => { break },

            "Print" => {
                println!("すべての雇用者を表示します");
                if v.len() > 2 {
                    println!("不必要な引数があります");
                    continue;
                }
                println!("----------------------");
                if v.len() == 1 {
                    for (department_name, members) in &employee_list {
                        println!("{}:", department_name);
                        for employee_name in members {
                            println!("    {}", employee_name);
                        }
                    }
                } else if v.len() == 2 {
                    let department_name = v[1].to_string();
                    match employee_list.get(&department_name) {
                        Some(v) => {
                            let mut vv = v.clone();
                            vv.sort();
                            println!("{}:", department_name);
                            for employee_name in vv {
                                println!("    {}", employee_name);
                            }
                        },
                        None => {
                            println!("{}はメンバーがいません", department_name);
                        }
                    }
                }
                println!("----------------------");
            },

            _ => {
                println!("不明なコマンドです");
            },
        }
    }
}

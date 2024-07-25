use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("スレッドから"),
            String::from("挨拶してるよ!"),
            String::from("やぁ!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // println!("送った値: {}", val)
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("もっと"),
            String::from("スレッドを増やして"),
            String::from("挨拶したいな!"),
            String::from("こんばんは!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("受け取った値: {}", received);
    }
}

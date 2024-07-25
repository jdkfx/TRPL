use std::thread;
use std::time::Duration;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} in thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle1.join().unwrap();

    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("vector: {:?}", v);
    });

    handle2.join().unwrap();

    for i in 1..5 {
        println!("Number {} in main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }
}

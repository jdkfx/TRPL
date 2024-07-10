use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    let _f1 = File::open("hello.txt");

    let _f1 = match _f1 {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "ファイルを作成しようとしましたが、問題がありました {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "ファイルを開く際に問題がありました {:?}",
                error
            )
        },
    };

    // let f2 = File::open("nice.txt").unwrap();
    let _f2 = File::open("nice.txt").expect("nice.txtがありません!");

    let _ = read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

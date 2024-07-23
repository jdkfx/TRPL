struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointerをデータ`{}`とともにドロップします!", self.data);
    }
}

fn main() {
    // let c = CustomSmartPointer { data: String::from("my stuff") };
    // let d = CustomSmartPointer { data: String::from("other stuff") };
    // println!("CustomSmartPointersが生成されました");

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointerが生成されました");
    drop(c);
    println!("mainの終端の前にCustomSmartPointerがドロップされました");
}

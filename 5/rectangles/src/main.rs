// 構造体を使う
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッドを使う
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // タプルを使う
    // let rect1 = (30, 50);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // println!("長方形の面積は{}平方ピクセルです", area(&rect1));
    println!("長方形の面積は{}平方ピクセルです", rect1.area());
    println!("rect1は{:#?}です", rect1);

    println!("rect1にrect2ははまり込む？ {}", rect1.can_hold(&rect2));
    println!("rect1にrect3ははまり込む？ {}", rect1.can_hold(&rect3));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// タプルを使う
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 構造体を使う
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
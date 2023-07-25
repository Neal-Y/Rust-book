// fn main() {
//     let w = 15;
//     let h = 20;
//     let area_result = area(w, h);

//     println!("{area_result}");
// }

// fn area(width: u32, height: u32) -> u32 {
//     return width * height;
// }
//? update_area

// fn main() {
//     let reac = (15, 20);
//     let area_result = area(reac);

//     println!("{area_result}");
// }

// fn area(dim: (u32, u32)) -> u32 {
//     return dim.0 * dim.1;
// }//? 可讀性降低了 因為參數的沒有名字

//? update_struct_area

// struct Rectangle {
//     height: u32,
//     width: u32,
// }
// fn main() {
//     let area_result = Rectangle {
//         height: 30,
//         width: 20,
//     };

//     println!("{}", area(&area_result));
// }

// fn area(rect: &Rectangle) -> u32 {
//     return rect.width * rect.height;
// }

//? impl 因為有些fn只適合特定的struct所以我們乾脆把他變成struct自己的fn
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height > other.height && self.width > other.width;
    }
}
fn main() {
    let area1 = Rectangle {
        height: 30,
        width: 50,
    };
    let area2 = Rectangle {
        height: 10,
        width: 40,
    };
    let area3 = Rectangle {
        height: 35,
        width: 55,
    };

    println!("{}", area1.can_hold(&area2));
    println!("{}", area2.can_hold(&area3));
}

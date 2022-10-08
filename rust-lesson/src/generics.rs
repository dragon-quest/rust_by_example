struct Point<T> {
    x: T,
    y: T,
}
struct PointAnother<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}
pub fn run() {
    let number_list = vec![32, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest is {}", largest);
    // println!("The largest is {}", largest_i32(number_list));
    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("The largest is {}", largest(char_list));
    println!("The largest is {}", largest(number_list));
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 5, y: 2.3 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("p5 {} {}", p5.x, p5.y);

    // 参照(reference)と借用(borrowing)
    // 借用：所有者を移動させずに参照する権利だけを貸し出す
    // ・参照（&によるアドレス）自体は、所有権が適応される型 + プリミティブ型両方に存在する
    // ・プリミティブ型は所有権の概念がないので借用という概念はない

    // Rustのメモリ安全性
    // 二重解放エラー -> 所有権システム
    // メモリ解放忘れ -> RAII(Resource Acquisition Is Initialization)
    // ダングリングポインタ -> ライフタイム
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

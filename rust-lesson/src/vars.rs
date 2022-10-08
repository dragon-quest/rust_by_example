const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!!");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    let y = 5;
    println!("Stack address of y is: {:p}", &y);

    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);

    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    {
        let y = 0;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("The value of t1 is: {} {} {}", x, y, z);
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a2[3]);

    // 文字列スライス
    // &str: 文字列スライス型
    // s2
    // ptr(8 bytes), len(8 bytes)
    // スライスは、先頭アドレスと要素数で表現される
    let s1 = "helloこんにちは挨拶"; // 5bytes + 21bytes = 26bytes : 静的領域
    let s2 = "hello";
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    println!("Static memory address of s1: {:?}", s1.as_ptr());
    println!("Static memory address of s2: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());

    // String型
    // Heap領域
    // データ所有 = メモリ解放(drop)をする責任を負う
    // ptr(8 bytes), len(8 bytes), cap(8bytes)
    let mut s1 = String::from("hello");
    let mut s2 = String::from("hello world");
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    println!("Heap memory address of s1: {:?}", s1.as_ptr());
    println!("Heap memory address of s2: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);

    // 所有権による二重解放エラー回避
    // 所有権者は、データに対して必ず一人
    // 所有権者がメモリを解放する
    // -> 解放(drop)はRustによって自動的に行われる

    // 文字列スライスはなぜデータを所有しないか？
    // 文字列リテラルから文字列スライスを作った場合は、文字列データは静的領域にあるのでそもそも解放する必要がない
    // String型から文字列スライスを作った場合は、所有権は、String型から移動しない

    // 参照と借用
    // 借用：所有権を移動させずに参照する権利だけを貸し出す
}

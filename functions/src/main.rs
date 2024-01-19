use std::mem::size_of;

fn abdul() -> i16 {
    let mut a: i16 = 10;
    let mut b: i16 = 90;
    println!(" value {}", b);
    b = 89;
    a = -90;

    return a + b;
}

fn main() {
    // let a: i32 = 900; // 2/2
    // a = 00X000999898, 00X000999899, 00X000999900, 00X000999901 = a -> [900] - 4 bytes
    println!("i8 type {} bytes", size_of::<i8>());
    println!("i16 type {} bytes", size_of::<i16>());
    println!("i32 type {} bytes", size_of::<i32>());
    println!("i64 type {} bytes", size_of::<i64>());
    println!("i128 type {} bytes", size_of::<i128>());
}

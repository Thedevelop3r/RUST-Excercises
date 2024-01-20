// fn sum_all(ar: &Vec<i32>) -> i32 {
// algorithm
//     println!("Algorithem started ->");
//     let mut sum: i32 = 0;
//     for i in ar {
//         println!("sum = {} , i = {}, sum + i = {} ", sum, i , sum+i);
//         sum = sum + i; // 0 + 2 = 2,  2 + 3 = 5, 5 + 4 = 9, 9 + 5 = 14
//     } //automatic loop end
//     return sum; // 14
// }

use std::io; // std -> io

fn table(t: i128) {
    // variable track loop only 10 times
    println!("Table of {}", t);
    let mut i: i128 = 1;
    loop {
        let result: i128 = t * i;
        println!("{} X {} = {:?}", t, i, result);
        i = i + 1;

        if i > 100 {
            break;
        }
    }
}

fn main() {
    loop {
        println!("Please enter a value between 1 to 127 for its table\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input"); // "37" -> 37
        let converted_value: i128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        table(converted_value);
        println!("-----Table finished-----");
    }
}

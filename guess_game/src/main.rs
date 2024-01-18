use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut count: u8 = 1;
    loop {
        count += 1;
        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();
        println!("\n\n\n-------------------");
        println!("Random u8: {}", n1);
        println!("Random u16: {}", n2);
        println!("Random u32: {}", rng.gen::<u32>());
        println!("Random i32: {}", rng.gen::<i32>());
        println!("Random float: {}", rng.gen::<f64>());
        println!("Random bool: {}", rng.gen::<bool>());
        println!("Random char: {}", rng.gen::<char>());
        println!("Random tuple: {:?}", rng.gen::<(i32, f64)>());
        println!("Random array: {:?}", rng.gen::<[i32; 3]>());

        if count == 30 {
            break;
        }
    }
}

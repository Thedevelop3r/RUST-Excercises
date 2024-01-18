use std::io;

fn add_number(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

fn subtract_number(n1: i32, n2: i32) -> i32 {
    return n1 - n2;
}

fn multiply_number(n1: i32, n2: i32) -> i32 {
    return n1 * n2;
}

fn divide_number(n1: i32, n2: i32) -> f32 {
    // convert n1, n2 to float
    let f_n1: f32 = n1 as f32;
    let f_n2: f32 = n2 as f32;
    return f_n1 / f_n2;
}

fn main() {
    let mut input_history: Vec<String> = Vec::new();
    let mut result: f32 = 0.0;
    let mut input = String::new();
    loop {
        println!("Calculator");
        println!("Example: 1 + 2 addition");
        println!("Example: 1 - 2 substraction");
        println!("Example: 1 / 2 division");
        println!("Example: 1 * 2 multiplication");
        println!("Enter your input: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parsed_input: String = input.trim().to_string();
        input_history.push(parsed_input.clone());

        let f_input: Vec<&str> = parsed_input.split(" ").collect();
        let n1: i32 = f_input[0].parse().unwrap();
        let n2: i32 = f_input[2].parse().unwrap();
        let operator: &str = f_input[1];

        if operator == "+" {
            result = add_number(n1, n2) as f32;
        } else if operator == "-" {
            result = subtract_number(n1, n2) as f32;
        } else if operator == "*" {
            result = multiply_number(n1, n2) as f32;
        } else if operator == "/" {
            result = divide_number(n1, n2);
            //  to fixed .2
            result = (result * 100.0).round() / 100.0;
        } else {
            println!("Invalid operator");
        }
        let len = input_history.len();
        input_history[len - 1] = format!("{} = {}", parsed_input, result)[..].to_string();

        println!("Result: {}", result);
        println!("Input History: {:?}", input_history);
        input.clear();
    }
}

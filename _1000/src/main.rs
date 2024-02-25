use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let numbers: Vec<&str> = buffer.split_whitespace().collect();
    let mut result: i32 = 0;

    for number_str in numbers {
        let number: i32 = number_str.trim().parse().unwrap();
        result += number;
    }

    println!("{result}");
}

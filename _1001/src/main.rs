use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let numbers: Vec<&str> = buffer.split_whitespace().collect();

    let a = numbers[0].parse::<i32>().unwrap();
    let b = numbers[1].parse::<i32>().unwrap();

    assert!(a > 0);
    assert!(b < 10);

    let result = a - b;

    println!("{result}");
}

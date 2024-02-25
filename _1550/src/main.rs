use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let decimal = u32::from_str_radix(buffer.trim().trim_start_matches("0x"), 16).unwrap();

    println!("{decimal}");
}

use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let buddhist_year: i32 = buffer.trim().parse().unwrap();
    let difference = 543;

    let western_year = buddhist_year - difference;

    println!("{western_year}");
}

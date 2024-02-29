use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let year: u32 = buffer.trim().parse().unwrap();

    let mut is_leaf_year = false;

    if year % 400 == 0 || (year % 100 != 0 && year % 4 == 0) {
        is_leaf_year = true;
    }

    if is_leaf_year {
        println!("1");
    } else {
        println!("0");
    }
}

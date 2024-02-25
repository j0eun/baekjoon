// 문제:
// 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.
//
// 입력:
// 첫째 줄에 A와 B가 주어진다. (0 < A, B < 10)
//
// 출력:
// 첫째 줄에 A+B를 출력한다.
//
// 예제 입력 1:
// 1 2
//
// 예제 출력 1:
// 3

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

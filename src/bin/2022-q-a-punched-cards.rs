use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let t: usize = read();
    for i in 0..t {
        testcase(i);
    }
}

fn testcase(i: usize) {
    println!("Case #{}:", i + 1);
    let r: usize = read();
    let c: usize = read();
    let mut first = vec!['.', '.', '+'];
    let mut second = vec!['.', '.', '|'];
    let mut odd = vec!['+'];
    let mut even = vec!['|'];
    for i in 0..c {
        if i != 0 {
            first.push('-');
            first.push('+');
            second.push('.');
            second.push('|');
        }
        odd.push('-');
        odd.push('+');
        even.push('.');
        even.push('|');
    }
    let first_str = first.iter().collect::<String>();
    let second_str = second.iter().collect::<String>();
    let odd_str = odd.iter().collect::<String>();
    let even_str = even.iter().collect::<String>();
    for i in 0..r {
        if i == 0 {
            println!("{}", first_str);
            println!("{}", second_str);
        } else {
            println!("{}", odd_str);
            println!("{}", even_str);
        }
    }
    println!("{}", odd_str);
}

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

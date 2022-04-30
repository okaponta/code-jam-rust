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
    print!("Case #{}: ", i + 1);
    todo!();
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

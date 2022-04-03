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

fn testcase(casenum: usize) {
    print!("Case #{}: ", casenum + 1);
    let mut p = vec![1 << 30; 4];
    for _ in 0..3 {
        for i in 0..4 {
            p[i] = p[i].min(read());
        }
    }
    if p.iter().sum::<i32>() < 1_000_000 {
        println!("IMPOSSIBLE");
        return;
    }
    let mut remain = 1_000_000;
    for i in 0..4 {
        if p[i] > remain {
            p[i] = remain;
            remain = 0;
        } else {
            remain -= p[i];
        }
    }
    println!(
        "{}",
        p.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
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

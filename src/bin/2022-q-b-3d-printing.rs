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
    let n: usize = read();
    let mut s = vec![0; n];
    for i in 0..n {
        let si: usize = read();
        s[i] = si;
    }
    s.sort();
    let mut ans = 1;
    for i in 0..n {
        if s[i] >= ans {
            ans += 1;
        } else {
            continue;
        }
    }
    println!("Case #{}: {}", casenum + 1, ans - 1);
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

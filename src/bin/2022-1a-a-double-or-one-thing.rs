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
    let s: String = read();
    let mut chs = vec![];
    let mut counts = vec![];
    let mut len = 0;
    let mut prev = '0';
    for c in s.chars() {
        if c == prev {
            counts[len - 1] += 1;
        } else {
            chs.push(c);
            counts.push(1);
            len += 1;
            prev = c;
        }
    }
    for i in 0..len - 1 {
        if chs[i + 1] as u8 > chs[i] as u8 {
            counts[i] *= 2;
        }
    }
    let mut ans = vec![];
    for i in 0..len {
        for _ in 0..counts[i] {
            ans.push(chs[i]);
        }
    }
    println!("{}", ans.iter().collect::<String>());
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

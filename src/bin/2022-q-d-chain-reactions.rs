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

    let n: usize = read();
    let mut f = vec![0; n + 1];
    for i in 0..n {
        let fi: usize = read();
        f[i + 1] = fi;
    }
    let mut rev_edges = vec![vec![]; n + 1];
    for i in 0..n {
        let pi: usize = read();
        rev_edges[pi].push(i + 1);
    }

    let ans = dfs(0, &rev_edges, &f);
    println!("{}", ans.0);
}

fn dfs(cur: usize, edges: &Vec<Vec<usize>>, fun: &Vec<usize>) -> (usize, usize) {
    if edges[cur].is_empty() {
        return (fun[cur], fun[cur]);
    }
    let mut min = 1 << 60;
    let mut sum = 0;
    for &next in edges[cur].iter() {
        let result = dfs(next, edges, fun);
        sum += result.0;
        min = min.min(result.1);
    }
    if min < fun[cur] {
        sum += fun[cur] - min;
        min = fun[cur];
    }
    (sum, min)
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

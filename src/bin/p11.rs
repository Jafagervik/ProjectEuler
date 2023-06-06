fn parse_grid() -> Vec<Vec<usize>> {
    include_str!("../p11.txt")
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve() -> usize {
    let mat: Vec<Vec<usize>> = parse_grid();

    let m = mat.len();
    let n = mat[0].len();

    let vert = |i: usize, j: usize| (i..i + 4).map(|e| mat[e][j]).product::<usize>();
    let hor = |i: usize, j: usize| (j..j + 4).map(|e| mat[i][e]).product::<usize>();
    let diag = |i: usize, j: usize| (0..4).map(|e| mat[i + e][j + e]).product::<usize>();
    let diagl = |i: usize, j: usize| (0..4).map(|e| mat[i + e][j - e]).product::<usize>();

    let mut res = 0;

    for i in 0..m {
        for j in 0..n {
            if i + 3 < m && j + 3 < n {
                res = res.max(diag(i, j));
            }

            if i + 3 < m {
                res = res.max(vert(i, j));
            }

            if j + 3 < n {
                res = res.max(hor(i, j));
            }

            if i + 3 < m && j as i32 - 3 >= 0 {
                res = res.max(diagl(i, j));
            }
        }
    }

    res
}

fn main() {
    let sol = solve();

    println!("Solution: {:?}", sol);
}

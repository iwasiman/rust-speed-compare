use std::env;
/*
$ rustc -V
rustc 1.62.1 (e092d0b6b 2022-07-16)

同じ処理を行う関数で比較する、
書籍『実践Rustプログラミング入門』 P422のサンプルコードを参照に行った。
$ cargo build
   Compiling rust-add_arrray v0.1.0 (/Users/xxx/projects/rust-speed-compare/rust-add_arrray)
    Finished dev [unoptimized + debuginfo] target(s) in 0.91s
$ time ./target/debug/rust-add_arrray 10000 10000
実行結果 100000000

real    0m6.125s
user    0m5.744s
sys     0m0.008s

$ cargo build --release
   Compiling rust-add_arrray v0.1.0 (/Users/xxx/projects/rust-speed-compare/rust-add_arrray)
    Finished release [optimized] target(s) in 1.46s
$ time ./target/release/rust-add_arrray 10000 10000
実行結果 100000000

real    0m0.423s (デバッグビルドの1/14) 2回目からは0．06秒前後まで  (PHPの1/4, 2回目からは1/29)
user    0m0.048s
sys     0m0.002s

$ time ./target/release/rust-add_arrray 100000 100000
実行結果 10000000000

real    0m4.866s
user    0m4.842s
sys     0m0.007s

$ time ./target/release/rust-add_arrray 1000000 1000000
Rust 実行結果 1000000000000

real    10m30.869s   100万件だとRustでも10分半かかる。
user    10m13.068s
sys     0m1.511s

$ time ./target/release/rust-add_arrray 2000000 2000000
Rust 実行結果 4000000000000

real    43m36.613s  200万件だと43分半。
user    42m20.085s
sys     0m7.737s

*/


/// メイン関数。
fn main() {
    let args: Vec<_> = env::args().collect();
    let n = args[1].parse::<u64>().unwrap();
    let x = args[2].parse::<u64>().unwrap();
    println!("Rust 実行結果 {}", add_array(n, x));
}


/// 配列の各要素に+1し、最後に合計数を返します。
/// param n: u64 配列の要素数
/// param x: u64 処理を繰り返す回数
/// returns u64 配列の各要素の合計値
/// 
fn add_array(n: u64, x: u64) -> u64 {
    // 引数n個の要素を持った配列(正確にはベクタ)を作り各要素は0に初期化
    let mut a = vec![0u64; n as usize];
    // 引数のx回、配列の全要素に+1していく
    for _ in 0 ..x {
        for i in 0..n as usize {
            a[i] += 1;
        }
    }
    // 合計値を返す
    a.iter().sum()
}
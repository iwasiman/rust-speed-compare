package main

import (
	"fmt"
	"os"
	"strconv"
)

/*
$ go version
go version go1.18 darwin/amd64

$ go build
$ time ./golang-add_array 10000 10000
Golangでの実行結果 100000000

real    0m0.544s  (0．22秒ぐらいのときもあった。2回めからは0.1を切って0.05-0.06)
user    0m0.044s
sys     0m0.004s

$ time ./golang-add_array 100000 100000
Golangでの実行結果 10000000000

real    0m4.706s  4.6ぐらいのときも。複数回やっても変わらない。
user    0m4.488s
sys     0m0.019s

$ time ./golang-add_array 1000000 1000000
Golangでの実行結果 1000000000000

real    10m5.680s 100万件、Goで10分5秒。
user    9m37.801s
sys     0m2.381s

$ time ./golang-add_array 2000000 2000000
Golangでの実行結果 4000000000000

real    45m10.163s 200万件、45分10秒
user    40m2.944s
sys     0m12.304s

*/

func main() {
	n, _ := strconv.Atoi(os.Args[1])
	x, _ := strconv.Atoi(os.Args[2])
	fmt.Println("Golangでの実行結果 " + strconv.Itoa(add_array(n, x)))
}

// 配列の各要素に+1し、最後に合計数を返します。
// param n int 配列の要素数
// param x int 処理を繰り返す回数
// returns int 配列の各要素の合計値
func add_array(n, x int) int {
	// 引数n個の要素を持った配列を作り各要素は0に初期化
	// arr := [n]int{} のようにしたいが初期サイズには定数しか使えないのでスライスにする
	list := make([]int, n)
	for i := range list {
		list[i] = 0
	}
	// 引数のx回、配列の全要素に+1していく
	for i := 0; i < x; i++ {
		for j := 0; j < n; j++ {
			list[j] += 1
		}
	}
	total := 0
	for j := 0; j < n; j++ {
		total += list[j]
	}
	// 合計値を返す
	return total
}

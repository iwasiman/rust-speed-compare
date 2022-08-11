/*
$ node -v
v16.16.0

$ time node add_array.js 10000 10000
JavaScriptでの実行結果 100000000

real    0m0.437s
user    0m0.163s
sys     0m0.083s

$ time node add_array.js 100000 100000
JavaScriptでの実行結果 10000000000

real    0m11.835s
user    0m11.560s
sys     0m0.092s

*/

/*
 * 配列の各要素に+1し、最後に合計数を返します。
 * @param int n 配列の要素数
 * @param int x 処理を繰り返す回数
 * returns int  配列の各要素の合計値
 */
 const add_array = function(n, x) {
  // 引数n個の要素を持った配列(正確にはベクタ)を作り各要素は0に初期化
  let arr = Array(n).fill(0);
  // 引数のx回、配列の全要素に+1していく
  for (let count = 0; count < x; count++){
    for (let index = 0; index < n; index++) {
      arr[index] += 1;
    }
  }
  // 合計値を返す
  let sum = arr.reduce((a, b) => {
    return a + b;
  });
  return sum;
};

const n = Number(process.argv[2]);
const x = Number(process.argv[3]);
console.log("JavaScriptでの実行結果 " + add_array(n, x));

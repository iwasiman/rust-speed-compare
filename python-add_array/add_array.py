import sys

'''
$ python3 --version
Python 3.9.13

$ time python3 add_array.py 10000 10000
Python3での実行結果 100000000

real    0m8.573s
user    0m8.467s
sys     0m0.025s

$ time python3 add_array.py 100000 100000
Python3での実行結果 10000000000

real    15m58.473s
user    14m17.617s
sys     0m3.486s
'''

# 配列の各要素に+1し、最後に合計数を返します。
# param n int 配列の要素数
# param x int 処理を繰り返す回数
# returns int 配列の各要素の合計値
def add_array(n, x):
    # 引数n個の要素を持った配列(正確にはリスト)を作り各要素は0に初期化
    ary = [0] * n
    # 引数のx回、配列の全要素に+1していく
    for i in range(x):
        for j in range(n):
            ary[j] += 1
    # 合計値を返す 新規に配列を0から舐めていくと若干遅かったのでsum()にする
    return sum(ary)

args = sys.argv
result = add_array(int(args[1]), int(args[2]))
print("Python3での実行結果 " + str(result))
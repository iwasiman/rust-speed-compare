import java.util.Arrays;
import java.math.*;

/*
$ java --version
openjdk 18.0.2 2022-07-19
OpenJDK Runtime Environment Homebrew (build 18.0.2+0)
OpenJDK 64-Bit Server VM Homebrew (build 18.0.2+0, mixed mode, sharing)

$ javac Main.java

$ time java Main 10000 10000
Javaでの実行結果 100000000

real    0m0.222s
user    0m0.154s
sys     0m0.019s

$ time java Main 100000 100000
Javaでの実行結果 10000000000

real    0m8.516s
user    0m8.080s
sys     0m0.045s

$ time java Main 1000000 1000000
Javaでの実行結果 1000000000000

real    14m22.989s
user    13m41.243s
sys     0m2.683s

*/


public class Main {
    public static void main(String[] args) {
        int n = Integer.parseInt(args[0]);
        int x = Integer.parseInt(args[1]);
        System.out.println("Javaでの実行結果 "+ String.valueOf(Main.addArray(n, x)));
    }

    /**
     * 配列の各要素に+1し、最後に合計数を返します。
     * @param int $n 配列の要素数
     * @param int $x 処理を繰り返す回数
     * returns BigInteger  配列の各要素の合計値
     */
    public static BigInteger addArray(int n, int x) {
        // 引数n個の要素を持った配列を作り各要素は0に初期化
        int[] ary = new int[n];
        Arrays.fill(ary, 0);
        // 引数のx回、配列の全要素に+1していく
        for (int count = 0; count < x; count++) {
            for (int index = 0; index < n; index++) {
                ary[index] += 1;
            }
        }
        // Stream APIで合計が出せるかと思ったが普通にやる
        //return Arrays.stream(ary).sum();
        // intでやると桁あふれしてくるので結果だけはBigInteger型で。
        BigInteger total = new BigInteger("0");
        for (int index = 0; index < ary.length; index++) {
            BigInteger theValue = new BigInteger(String.valueOf(ary[index]));
            total = total.add(theValue);
        }
        return total;
    }
}


<?php

/*
$ php --version
PHP 7.4.30 (cli) (built: Jun  9 2022 09:30:03) ( NTS )
Copyright (c) The PHP Group
Zend Engine v3.4.0, Copyright (c) Zend Technologies
    with Zend OPcache v7.4.30, Copyright (c), by Zend Technologies

$ time php add_array.php 10000 10000
実行結果 100000000

real    0m4.788s 複数回やってもだいたいこのへん
user    0m4.283s
sys     0m0.019s

$ time php add_array.php 100000 100000
実行結果 10000000000

real    7m56.201s
user    7m18.007s
sys     0m1.616s
*/

echo "実行結果 " . add_array($argv[1], $argv[2]) . PHP_EOL;

/**
 * 配列の各要素に+1し、最後に合計数を返します。
 * @param int $n 配列の要素数
 * @param int $x 処理を繰り返す回数
 * returns int  配列の各要素の合計値
 */
function add_array($n, $x) {
    $array = [];
    for ($i = 0; $i < $n; $i++) {
        $array[$i] = 0;
    }
    for ($count = 0; $count < $x; $count++) {
        for ($idx = 0; $idx < $n; $idx++) {
            $array[$idx] += 1;
        }
    }
    $total = 0;
    for ($idx = 0; $idx < $n; $idx++) {
        $total += $array[$idx];
    }
    return $total;
}
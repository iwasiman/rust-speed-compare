# rust-speed-compare
同じ処理をする関数でRust言語と他の言語で速度比較をしてみたサンプルです。


|引数n,x|Ruby 2.6.8|Python 3.9.13|PHP 7.4.3|Java 18.0.2|Go 1.18|Rust 1.62.1|めも|
|:---|:---|:---|:---|:---|:---|:---|
|**1万**|6.862s|8.573s|4.788s|4.767s|0.544s|0.423s|RustはRubyの約16倍、Pythonの20倍、PHPの約11倍を達成。Goとほぼ互角。Javaはint型を使えば1秒を切るが、後ほど桁あふれするのでBigInteger使用。|
|**10万**|11m26.039s|15m58.473s|7m56.201s|7m8.722s|4.706s|4.866s|RustはRubyの約140倍、Pythonの約170倍、PHPの約98倍を達成。ここでもGoとほぼ互角。Javaはint型を使えば9sだが桁あふれ。|
|**100万**|未測定|未測定|未測定|未測定|xxx{10m5.680s |10m13.068s|僅差でGoの方が速かった。|
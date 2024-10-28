# AsMacro
AsMacroとは、低レイヤ向けのアセンブラ言語であり、高度なプリプロセッサーを持つことにより、
高水準言語の便利さとアセンブリの柔軟性、高速なビルド、高速な実行、高水準言語と同じ機能で効率の良い開発が可能という特徴を持つ予定です。  
## コード例
```
fn main() -> u64 {
    let a: u64 = 12345;
    a * a
}
```
## 内部動作
高度な機能を持つプリプロセッサ -> リンカ兼アセンブラ

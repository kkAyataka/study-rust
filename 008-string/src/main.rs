fn main() {
    // char
    let c = 'A'; // 32-bit UnicodeだけどUnicode code pointではない
    let cu8 = c as u8;
    println!("{}", cu8); // u8にキャスト
    println!("{}", cu8 as char);

    // string
    let s = "abあいうえ";         // UTF-8
    println!("{}", &s[0..2]);           // 部分文字列の取り出し
    // println!("{}", &s[3..]);         // byteでの切り出しなので、日本語には対応しない
    println!("{:?}", s.chars().nth(2)); // charsで分解できる

    // &str <-> string
    let cs: &str = "固定"; // 固定文字列
    let ms: String = String::from("可変"); // 可変文字列
    let cs2ms = cs.to_string();
    let ms2cs = ms.as_str();
    println!("{} {}", cs2ms, ms2cs);
}

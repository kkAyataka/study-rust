fn main() {
    // 「;」で終わるのが文 (statement)
    let _a = 0;

    // 値を持つのが式 (expression)
    let _v = 1 + 2; // 1 + 2は式
    let _v = { 1 + 2 }; // ブロックも式 (内部に;がなければ
    let _v = if false { 100 } else { 10 }; // ifも式
    println!("{}", _v);
}

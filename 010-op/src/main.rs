fn main() {
    // 四則演算
    println!("{}", 1 + 2);
    println!("{}", 2 * 2);
    println!("{}", 2 / 3);
    println!("{}", 2.0 / 3.0);
    println!("{}", 2 % 3);
    println!("{}", 2.0 % 3.0);

    // ビット演算
    println!("{:0>4b}", 0b1001 & 0b1100);
    println!("{:0>4b}", 0b1001 | 0b1100);
    println!("{:0>4b}", 0b1001 ^ 0b1100);
    println!("{:0>4b}", !0b1001);

    // シフト演算子
    println!("{:0>4b}", 0b1001 << 1);
    println!("{:0>4b}", 0b1001 >> 1);

    // 論理演算子
    println!("{}", true && true);
    println!("{}", false || false);
}

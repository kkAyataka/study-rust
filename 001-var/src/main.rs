fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let _s1 = "string"; // 型はコンパイラに推測させる
    let _s2: &str = "string"; // 明示する場合は「:」の後に書く

    // 整数型
    let _ni8: i8; let _nu8: u8;     // 8-bit
    let _ni16: i16; let _nu16: u16; // 16-bit
    let _ni32: i32; let _nu32: u32; // 32-bit
    let _ni64: i64; let _nu64: u64; // 64-bit
    let _nisize: isize; let _nusize: usize; // 処理系依存
    let _nix1 = 1; // i32
    let _nix2 = 1_000_000_000_000i64; // 自動で拡張しないので明示が必要
    print_typename(_nix1);
    print_typename(_nix2);

    // 数値リテラル
    let _nl10 = 12_345;     // 10進数。「_」で桁区切りを表現できる
    let _nl16 = 0xFF;       //16進数
    let _nl8 = 0o77;        // 8進数
    let _ul2 = 0b1110_0001; // 2進数
    let _ulb: u8 = b'A';    // 1バイト。型はu8

    // 浮動小数展型
    let _nf16: f32 = 12.34;
    let _nf32: f64 = 12.34;
    let _nf = 12.34; // f64

    // 論理型
    let _t: bool = true;
    let _f: bool = false;

    // 文字・文字列
    let _c1: char = 'c'; // 32-bit Unicode
    let _c2: char = 'あ';
    let _c3: char = '🍺';
    let _s1: &str = "string";
    let _s2: String = String::from("Hello");
}

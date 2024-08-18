trait CalcArea {
    fn area(&self) -> u32;

    // デフォルト実装を指定できる
    fn print() -> String {
        String::from("面積")
    }
}

struct Rect {
    w: u32,
    h: u32,
}

impl CalcArea for Rect {
    fn area(&self) -> u32 {
        self.w * self.h
    }
}

// 原理的に既存のstructにも機能を足せる
trait ToInt {
    fn to_int(&self) -> u32;
}
impl ToInt for String {
    fn to_int(&self) -> u32 {
        match &self.parse::<u32>() {
            Ok(n) => *n,
            Err(e) => 0,
        }
    }
}

fn main() {
    let r = Rect {w: 10, h: 2};
    println!("{}", r.area());
}

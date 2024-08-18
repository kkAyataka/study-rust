static mut GLOBAL_VAR: u32 = 100;

// #[repr(C)] // C言語とメモリ配置が同じになる
#[derive(Debug)]
struct Struct {
    id: u32,
    name: String,
}

impl Struct {
    // &selfがなければクラスメソッド
    fn new(name: &str) -> Struct {
        let id = unsafe {
            GLOBAL_VAR += 1;
            GLOBAL_VAR
        };
        Struct {
            id: id,
            name: String::from(name),
        }
    }

    fn print(&self) {
        println!("{} {}", self.id, self.name);
    }

    fn reset(&mut self) {
        self.id = 0;
        self.name = String::from("");
    }
}

// デフォルトmoveなので、必要に応じて参照に
fn print(s: &Struct) {
    println!("{:?}", s);
}

fn main() {
    let s = Struct {
        id: 100,
        name: String::from("name"),
    };
    println!("{} {}", s.id, s.name);
    println!("{:?}", s);

    s.print();

    let mut s = Struct::new("test");
    println!("{:?}", s);
    s.reset();
    println!("{:?}", s);

    // 関数の引数
    print(&s);

    // フィールド名のない構造体
    struct Color(u8, u8, u8);
    let c = Color(0xFF, 0xFF, 0xFF);
    println!("{} {} {}", c.0, c.1, c.2);


}

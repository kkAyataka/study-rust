fn hello() {
    println!("hello");
}

fn plus(a: u32, b: u32) -> u32 {
    a + b
}

// 固定文字列、borrow
fn printx(s: &str) {
    println!("{}", s);
}

// 文字列を返す
fn add_world(s: &str) -> String {
    s.to_string() + " world"
}

fn vec_at(v: &Vec<i32>, i: usize) -> i32 {
    v[i]
}

fn main() {
    hello();
    println!("{}", plus(10, 20));
    printx("test");
    println!("{}", add_world("hello"));
    println!("{}", vec_at(&vec![1, 2, 3], 2));
}

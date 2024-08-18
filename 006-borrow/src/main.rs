// &で参照にして、moveからborrowにする
fn string_len(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("Hello");
    println!("{}", string_len(&s));
    println!("{}", s);
}

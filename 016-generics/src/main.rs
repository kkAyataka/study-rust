// where Tでトレイトを指定して制限する
fn print_array<T>(a: &[T]) where T: std::fmt::Debug {
    for i in a {
        print!("{:?} ", i);
    }
    println!("");
}

fn main() {
    print_array(&[1, 2, 3]);
    print_array(&["a", "b", "c"]);
}

fn main() {
    // Module std::fmt: https://doc.rust-lang.org/std/fmt/index.html

    println!("{}", format!("Hello")); // Create String
    println!("{}", format!("位置: {1} {1} {}", 1, 2));
    println!("{}", format!("名前: {n} {n}", n="name"));

    println!("{}", format!("Fill: {:0>2}", 1));
    println!("{}", format!("精度: {:.10}", 3.14));

    println!("{}", format!("Debug: {:} {:?} {:#?}", 1, (3, 4), (5, 6))); // Display、Debug、Pretty Debug
}

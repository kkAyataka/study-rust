fn main() {
    // copy
    let n1 = 100;
    let n2 = n1; // copy
    println!("{} {}", n1, n2);

    // copy
    let b1 = true;
    let b2 = b1; // copy
    println!("{} {}", b1, b2);

    // copy
    let f1 = 3.14;
    let f2 = f1; // copy
    println!("{} {}", f1, f2);

    // copy
    let c1 = 'x';
    let c2 = c1; // copy
    println!("{} {}", c1, c2);

    // copy
    let t1 = (1, '2');
    let t2 = t1; // copy
    println!("{:?} {:?}", t1, t2);

    // move
    let _s1 = String::from("Hello");
    let _s2 = _s1; // move
    // println!("{}", _s1); // s1はmoveしたので無効 (コンパイルエラー)
    println!("{}", _s2); // borrow
    println!("{}", _s2);
}

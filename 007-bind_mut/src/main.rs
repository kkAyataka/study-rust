fn main() {
    let n = 10; // 束縛
    println!("{}", n);

    let n = 'A'; // 代入はできないが、別の値に再束縛できる。シャドーイング
    println!("{}", n);

    let mut v = 100; // 代入可能なようにはmutを使う
    println!("{}", v);
    v = 102;
    // v = 3.14; 異なる型は入らない
    println!("{}", v);
}

fn main() {
    // 配列
    let a = [1, 2, 3, 4]; // 配列。サイズは固定
    println!("{} {}", a[0], a[3]);
    println!("{:?}", a);
    // let a4 = a[4]; // サイズ外はビルドエラー
    println!("{:?}", &a[2..4]); // スライス

    // i32の配列の型は[i32]で、[i32; 4]でサイズ4の型になる
    let a: [i32; 4] = [101; 4]; // 101でうめたサイズ4の配列
    // a[0] = 10; // letは配列が保持する値も変更できない
    println!("{:?}", a);

     // 配列のメモリ領域は連続している -> 数値型にキャスト可能
     let a = [4u8, 0u8, 0u8, 0u8];
     unsafe {
        let v = std::mem::transmute::<[u8;4], i32>(a);
        println!("{}", v);
     }

    // Vec
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    println!("{:?}", v1);

    // vec!
    let v2 = vec!['a', 'b', 'c'];
    println!("{:?}", v2);

    println!("{}", v2[1]); // 添字でアクセス可能
    println!("{:?}", v2.get(1)); // Optionが返る
}

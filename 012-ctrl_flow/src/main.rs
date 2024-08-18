fn main() {
    // if
    let c = 1u32;
    if c < 10 {
        println!("10未満");
    } else if c < 20 {
        println!("20未満");
    } else {
        println!("20以上");
    }

    // ifで値を返して受ける
    let c = 20u32;
    let r = if c < 10 {
        c + 1
    } else if c < 20 {
        c + 2
    } else {
        c + 3
    };
    println!("{}", r);


    // for (範囲)
    let a = [11, 12, 13];
    for i in a {
        println!("{}", i);
    }
    println!("{:?}", a);

    let v = vec![21, 22, 23];
    for i in &v { // &にないとmoveで渡される
        println!("{}", i);
    }
    println!("{:?}", v); // forで&がないとコンパイルエラー

    // for (イテレーター)
    for i in v.iter() {
        println!("{}", i);
    }

    // for (インデックス付き)
    for (i, v) in v.iter().enumerate() {
        println!("{}:{}", i, v);
    }

    // for (数)
    for i in 10..13 {
        if i == 10 {
            continue;
        }
        println!("{}", i);
        break;
    }


    // while
    let mut i = 0;
    while i < 3 {
        println!("while: {}", i);
        i += 1;
    }


    // 無限ループ
    let mut i = 0;
    loop {
        if i > 3 {
            break;
        }

        println!("loop: {}", i);
        i += 1;
    }

}

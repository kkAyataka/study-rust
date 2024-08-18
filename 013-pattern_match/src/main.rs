#[derive(Debug)]
enum LANG {
    JA = 0,
    EN,
    ZH_CN,
}

fn main() {
    // enum
    let l = LANG::EN;
    let m = match l {
        LANG::JA => "日本語",
        LANG::EN => "英語",
        _ => "その他",
    };

    println!("{:?}", l);
    println!("{}", m);


    // option
    let o = Some(-100);
    let v = match o {
        Some(i) => i,
        None => -1,
    };
    println!("{:?}, {}", o, v);

    // if letを使う
    if let Some(i) = o {
        println!("{}", i);
    };


    // 数値でmatch
    let v = 3;
    let m = match v {
        1 => "いち",
        2 => "に",
        3..=5 => "3〜5",
        // 3..5 => "3〜4", // これはexperimental
        _ => "以外",
    };
    println!("{}", m);

    // 文字列でmatch
    let v = "one";
    let m = match v {
        "one" => 1,
        _ => -1,
    };
    println!("{}", m);
}

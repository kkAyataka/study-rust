fn func(v: u32) -> Result<u32, std::fmt::Error> {
    if v > 0 {
        Ok(v)
    } else {
        Err(std::fmt::Error)
    }
}

// エイリアス
type Res1<T> = std::result::Result<T, std::fmt::Error>;
fn func2(v: u32) -> Res1<u32> {
    Ok(v*100)
}

// コンビネーター
fn res_combinater(s: &str) -> Result<i32, std::num::ParseIntError> {
    // Resultのmapメソッドを使うと、
    // OKの処理を定義しつつ、Errの場合は、そのままErrを返す
    s.parse::<i32>().map(|n| n * 2)
}

// Error delegation
fn res_delegation(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = s.parse::<i32>()?; // ?を使うと、エラーの場合は、Errで終了する
    Ok(n)
}

// except
// Error delegation
fn res_expect(s: &str) -> Result<i32, std::num::ParseIntError> {
    // expectはエラー時にプログラムを終了する
    let n = s.parse::<i32>().expect("Invalid Error");
    Ok(n)
}

fn main() {
    let r = func(1);
    match r {
        Ok(i) => println!("v: {}", i),
        Err(e) => println!("Err: {}", e),
    }

    let _r = func(1).unwrap(); // 値を直接取り出す。エラーの場合panicになる

    func2(1).unwrap();

    res_combinater("1").unwrap();
    res_delegation("1").unwrap();
    res_expect("1").unwrap();
}

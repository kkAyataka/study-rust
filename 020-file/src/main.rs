use std::{fs, io::{BufWriter, Read, Write}};

fn main() {
    // ファイルを読む
    let r = fs::read_to_string("./data/sample.txt");
    if let Ok(data) = r {
        println!("{}", data);
    } else {
        println!("{:?}", r.err());
    }

    // ファイルを開く
    let r = fs::File::open("./data/sample.txt");
    if let Ok(mut f) = r {
        let mut buf = String::new();
        if let Ok(_s) = f.read_to_string(&mut buf) {
            println!("{}", buf);
        }
    }

    // ファイルに書き出す
    let exe_path = std::env::current_exe().unwrap();
    let dir = exe_path.parent().unwrap();
    let path = dir.join("file.txt");
    fs::write(path.to_str().unwrap(), "テスト").unwrap();

    // 標準出力に書き出す
    let mut writer = BufWriter::new(std::io::stdout());
    let b = [0x41u8; 10];
    writer.write(&b).expect("error");
}

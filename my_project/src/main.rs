fn main() {
    println!("プログラミング課題");
    no1();
    no2();
}

// 課題１　重複しない乱数を生成
use rand::Rng; //乱数生成するためのトレイト
use std::collections::HashSet; //重複しない要素を保持する標準ライブラリ

fn no1() {
    // どういった乱数を生成するか決定している(3種類くらいある中でどれを使うか)
    let mut rng: rand::prelude::ThreadRng = rand::rng(); // 乱数生成器を作成

    // HushSetは重複したものを格納しないらしい
    let mut numbers: HashSet<u32> = HashSet::new();

    // 決めた乱数生成方法から実際に数字を決めるコード
    while numbers.len() < 5 {
        // 重複しない数字が5個になるまでループ
        let random_number: u32 = rng.random_range(0..=100);
        numbers.insert(random_number); // HashSet に挿入（重複すると挿入されない）
    }
    println!("ランダムな数字:  {:?}", numbers);
}

// 課題2　ファイルに出力
use std::fs::File;// ファイルを扱うための標準ライブラリ
use std::io::Write;// ファイルに書き込むために必要トレイト

fn no2() {

    //出力するファイル名を決定＆定義
    let path: &str = "write.txt";

    // file::createでファイル作成
       let mut newfile: File = File::create(path)
        // エラー文
           .expect("file not found.");
       
    // witenlnで作成したファイルに書き込んでいる   
       writeln!(newfile, "hello world.")
           .expect("cannot write.");
    
    print!("{:?}に書き込まれました",path)
}

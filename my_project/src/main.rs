fn main() {
    println!("プログラミング課題");
    no1();
    no2();
    no3();
}

// 課題１　重複しない乱数を生成
use rand::seq::SliceRandom; //乱数生成するためのトレイト＋スライスに関するトレイト

fn no1() {
    // どういった乱数を生成するか決定している(3種類くらいある中でどれを使うか)
    let mut randm = rand::rng(); // 乱数生成器を作成

    println!("1～100の範囲で乱数の個数を入力してください:");

    // 入力を受け取る
    let mut serectnum = String::new();
    std::io::stdin()
        .read_line(&mut serectnum)
        .expect("読み取りに失敗しました");

    //入力を数値に型変更
    let num_count: usize = match serectnum.trim().parse() {
        // 入力が指定の範囲かチェック
        Ok(n) if n > 0 && n < 10000001 => n,
        _ => {
            println!("無効な入力です。1～1000000以内の正の整数を入力してください。");
            return;
        }
    };

    // 数字を用意
    let mut numbers: Vec<u32> = (0..=1000000).collect();
    // 生成した数字を乱数生成器を用いてランダムに並べ替え
    numbers.shuffle(&mut randm);

    // 入力された数字の分、並べ替えた数字から取り出し
    let selected_numbers: Vec<u32> = numbers.into_iter().take(num_count).collect();

    println!("生成されたランダムな数字: {:?}", selected_numbers);
}

// 課題2　ファイル出力
use std::fs::File; // ファイルを扱うための標準ライブラリ
use std::io::Write; // ファイルに書き込むために必要トレイト

fn no2() {
    //出力するファイル名を決定＆定義
    let path: &str = "write.txt";

    // file::createでファイル作成
    let mut newfile: File = File::create(path)
        // エラー文
        .expect("file not found.");

    // witenlnで作成したファイルに書き込んでいる
    writeln!(newfile, "hello world.").expect("cannot write.");

    println!("{:?}に書き込まれました", path)
}

// 課題3　ファイル入力
fn no3() {
    println!("ファイル名を入れてください:");

    let mut filename = String::new();
    std::io::stdin()
        .read_line(&mut filename)
        .expect("読み取りに失敗しました");

    // 改行文字を取り除く
    // stdin から読み取った入力には末尾に改行が含まれるらしい
    let filename = filename.trim();

    // ファイルを作成
    let mut createfile: File =
        File::create(filename).expect(&format!("ファイル '{}' の作成に失敗しました", filename));

    println!("ファイル内に記載するテキストを入力してください:");

    let mut newtext = String::new();
    std::io::stdin()
        .read_line(&mut newtext)
        .expect("テキストの入力に失敗しました");

    // newtext.trim()で入力した文字の末尾の改行を防ぐ
    writeln!(createfile, "{}", newtext.trim()).expect("ファイルへの書き込みに失敗しました");

    println!("テキストがファイル '{}' に書き込まれました", filename);

}

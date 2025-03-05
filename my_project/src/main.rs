fn main() {
    println!("プログラミング課題");
    no1();
    no2();
    no3();
    no4();
    no5();
    no6();
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
    let refilename = filename.trim();

    // ファイルを作成
    let mut createfile: File =
        File::create(refilename).expect(&format!("ファイル '{}' の作成に失敗しました", filename));

    println!("ファイル内に記載するテキストを入力してください:");

    let mut newtext = String::new();
    std::io::stdin()
        .read_line(&mut newtext)
        .expect("テキストの入力に失敗しました");

    // newtext.trim()で入力した文字の末尾の改行を防ぐ
    writeln!(createfile, "{}", newtext.trim()).expect("ファイルへの書き込みに失敗しました");

    println!("テキストがファイル '{}' に書き込まれました", refilename);
}

// 課題４　ソート

use std::io::{BufRead, BufReader}; // ファイルを読むために必要なトレイル

fn no4() {
    println!("ソートするファイル名を入れてください:");

    let mut searchfilename = String::new();
    std::io::stdin()
        .read_line(&mut searchfilename)
        .expect("読み取りに失敗しました");
    let researchfilename = searchfilename.trim();

    // 指定したファイル名のファイルを開く
    // my_project内似なければいけない
    let openfile = File::open(researchfilename).expect("ファイルを開くことができませんでした");

    // 1行ずつ読み取るために必要
    let buffered = BufReader::new(openfile);

    // 読み取ったテキストをベクター型に格納
    let mut contents: Vec<String> = buffered
        .lines() // `lines()` は `Result<String>` を返す
        .filter_map(|line| line.ok()) // エラーを無視し、`Ok(String)` のみ取得
        .collect();

    contents.sort();

    println!("ソート結果{:?}", contents);
}

// 課題5
use std::collections::HashMap;
use std::io::Read;

fn no5() {
    println!("単語をソートするファイル名を入れてください:");

    let mut cont_searc_filename = String::new();
    std::io::stdin()
        .read_line(&mut cont_searc_filename)
        .expect("読み取りに失敗しました");

    let re_cont_searc_filename = cont_searc_filename.trim();

    // 指定したファイル名のファイルを開く
    let mut opencountfile =
        File::open(re_cont_searc_filename).expect("ファイルを開くことができませんでした");

    // ファイル内のテキストを保持
    let mut content_text = String::new();

    // テキストファイルの読み込み
    opencountfile
        .read_to_string(&mut content_text)
        .expect("ファイルの読み込みに失敗しました");

    // 単語のカウント
    let mut word_count = HashMap::new();
    for words in content_text.split_whitespace() {
        *word_count.entry(words).or_insert(0) += 1;
    }

    // 出現回数でソートするために Vec に変換
    let mut sorted_counts: Vec<(String, u32)> = word_count
        .clone()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1)); // 出現回数で降順にソート

    println!("単語の出現回数:");
    for (words, count) in &sorted_counts {
        println!("{}: {}", words, count);
    }
}

// 課題6

use std::f64;
fn no6() {
    let a = -0.2; // 放物線の開き具合（絶対値を大きくすると急なカーブ）
    let width = 40; // コンソールの幅
    let height = 20; // コンソールの高さ（Y軸の最大値）
    // width を usize から i32 に変換してから - を適用
    let width_i32 = width as i32;

    let mut grid = vec![vec![' '; width]; height];

    for x in -width_i32 / 2..=width_i32 / 2 {
        let xf = x as f64;
        let yf = a * xf * xf; // y座標の計算
        let y = (height as i32 - 1 - yf.round() as i32).clamp(0, height as i32 - 1);

        let x_pos = (x + width_i32 / 2) as usize;
        if x_pos < width {
            grid[y as usize][x_pos] = '*';
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

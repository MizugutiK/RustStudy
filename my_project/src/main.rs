fn main() {
    println!("プログラミング課題");
    no1();
    no2();
    no3();
    no4();
    no5();
    no6();
    no7();
    no8();
}

// 課題１　重複しない乱数を生成
use rand::seq::SliceRandom; //乱数生成するためのトレイト＋スライスに関するトレイト

fn no1() {
    // どういった乱数を生成するか決定している(3種類くらいある中でどれを使うか)
    let mut randm = rand::rng(); // 乱数生成器を作成
    let mut input = String::new();

    // 入力ループ
    let num_count: usize = loop {
        println!("1～1,000,000以内の正の整数を入力してください。（終了する場合はCtrl+C）:");

        // 入力を受け取る
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("読み取りに失敗しました。もう一度入力してください。");
            continue;
        }
        // 数字のチェック
        match input.trim().parse::<usize>() {
            Ok(n) if (1..=1_000_000).contains(&n) => break n,
            _ => println!("無効な入力です。1～1,000,000の範囲の正の整数を入力してください。"),
        }
    };
    // 数字を用意
    let mut numbers: Vec<u32> = (1..=num_count as u32).collect();

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
    println!("読み込むファイル名を入れてください:");

    let mut filename = String::new();
    std::io::stdin()
        .read_line(&mut filename)
        .expect("読み取りに失敗しました");

    // 改行文字を取り除く
    // stdin から読み取った入力には末尾に改行が含まれるらしい
    let refilename = filename.trim();

    // ファイルを開く
    let open_file: File = File::open(refilename).expect(&format!(
        "ファイル '{}' を開くことに失敗しました",
        refilename
    ));

    // println!("ファイル内に記載するテキストを入力してください:");

    // let mut newtext = String::new();
    // std::io::stdin()
    //     .read_line(&mut newtext)
    //     .expect("テキストの入力に失敗しました");

    // // newtext.trim()で入力した文字の末尾の改行を防ぐ
    // writeln!(createfile, "{}", newtext.trim()).expect("ファイルへの書き込みに失敗しました");

    // let mut content_open = String::new();

    // open_file
    //     .read_to_string(&mut content_open)
    //     .expect("ファイルの読み込みに失敗しました");

    let opem_buffered = BufReader::new(open_file);

    //表示用
    for line in opem_buffered.lines() {
        match line {
            Ok(content) => println!("{}", content), // そのまま表示
            Err(e) => eprintln!("ファイルの読み込み中にエラーが発生しました: {}", e),
        }
    }
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
    println!("出現単語をカウントするファイル名を入れてください:");

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
    let a = 0.1; // 放物線の開き具合（絶対値を大きくすると急なカーブ）
    let width = 80;
    let height = 40;

    for y in -height / 2..=height / 2 {
        let yf = y as f64;
        let xf = a * yf * yf; // y座標の計算
        let x = width - 1 - (xf as i32); // 座標変換

        // インデントを調整して * を表示
        let spaces = " ".repeat(x as usize);
        println!("{}*", spaces);
    }
}

// 課題7
fn no7() {
    let path_out: &str = "output.txt";
    let mut new_output_file: File = File::create(path_out)
        // エラー文
        .expect("file not found.");
    let a2 = 0.1; // 放物線の開き具合（絶対値を大きくすると急なカーブ）
    let height_second = 40;
    let width_second = 80;

    for y in -height_second / 2..=height_second / 2 {
        let yf = y as f64;
        let xf = a2 * yf * yf; // y座標の計算
        let x = width_second - 1 - (xf as i32); // 座標変換
        // インデントを調整して * を表示
        let input_spaces = " ".repeat(x as usize);

        writeln!(new_output_file, "{}*", input_spaces).expect("cannot write.");
    }

    // witenlnで作成したファイルに書き込んでいる
    println!("{:?}ファイルに放物線が記載されました", path_out);
}

// 課題8
fn no8() {
    // どういった乱数を生成するか決定している(3種類くらいある中でどれを使うか)
    let mut randm = rand::rng(); // 乱数生成器を作成

    println!("数あてゲーム");
    println!("1～100の数字を入れてください(-1で強制終了)");

    // 数字を用意
    let mut numbers: Vec<u32> = (1..=100).collect();

    // 生成した数字を乱数生成器を用いてランダムに並べ替え
    numbers.shuffle(&mut randm);

    // 入力ループ
    loop {
        // 入力された数字の分、並べ替えた数字から取り出し
        let selected_number = numbers[0];
        let mut input: String = String::new(); //入力された数字保持

        std::io::stdin().read_line(&mut input).expect("入力エラー");

        if input.trim() == "-1" {
            println!("ゲームを終了します。正解の数字: {}", selected_number);
            break;
        }

        // 数字のチェック
        let guess: u32 = match input.trim().parse() {
            Ok(num) if (1..=100).contains(&num) => num,
            _ => {
                println!("無効な入力です。1～100の範囲で入力してください。");
                continue;
            }
   
        };

        // 数字判定部分
        if selected_number > guess {
            println!("数字が小さいです")
        } else if guess > selected_number {
            println!("数字が大きいです")
        } else {
            println!("当たりました！ 🎉 正解の数字: {}", selected_number);
            break ;
        }
    }
}

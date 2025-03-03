use rand::Rng;
use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
    randm();
}

// 課題１　重複しない乱数を生成
fn randm(){
    // どういった乱数を生成するか決定している(3種類くらいある中でどれを使うか)
    let mut rng = rand::rng(); // 乱数生成器を作成
    
    // HushSetは重複したものを格納しないらしい
    let mut numbers = HashSet::new();
    
    // 決めた乱数生成方法から実際に数字を決めるコード
    while numbers.len() < 5 { // 重複しない数字が5個になるまでループ
        let random_number: u32 = rng.random_range(0..=100);
        numbers.insert(random_number); // HashSet に挿入（重複すると挿入されない）
    }    
    println!("ランダムな数字:  {:?}", numbers);
}
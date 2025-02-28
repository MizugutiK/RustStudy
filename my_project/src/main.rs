use rand::Rng;

fn main() {
    println!("Hello, world!");
    randm();
}

// 課題１　重複しない乱数を生成
fn randm(){
    // どういった乱数を生成するか決定している(3種類くらいある中でどれを使うか)
    let mut rng = rand::rng(); // 乱数生成器を作成

    // 決めた乱数生成方法から実際に数字を決めるコード
    let random_number: u32 = rng.random_range(0..=100); // 0〜100の乱数を生成
    println!("ランダムな数字: {}", random_number);
}
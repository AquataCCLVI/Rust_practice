fn main() {
    // 可変配列
    let a = vec![1, 2, 3];

    println!("{:?}", a);

    // 空の可変配列
    let mut b: Vec<usize> = Vec::new();

    for i in 1..10 {
        //末尾に追加
        b.push(i);
    }

    println!("{:?}", b);
    //指定したインデックスを削除
    let c = b.remove(0);
    //末尾を削除 unwrapは空のときはクラッシュするので注意 unwrap_or(値)で空のときのケースを記述可能
    let d = b.pop().unwrap();
    //インデックス, 値
    b.insert(3, 10);

    println!("{:?}", b);
    //remove, popは戻り値を受け取れる
    println!("{}, {}", c, d);
    //インデックス指定で取得可能
    println!("{}", b[2]);
    //その他sort(),reverse(),contains(値)など用意されている
}

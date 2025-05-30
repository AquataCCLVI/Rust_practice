fn main() {
    //所有権
    //Rustのあらゆるものは所有権とよばれるもので制御されている
    //各変数に持つことができる所有者は1人だけ
    //所有者がスコープから外れると値は破棄される
    //これらの条件を静的にチェックすることにより、並列処理に優れた実行ファイルを生成できる

    let s1 = String::from("hello"); // 変数s1は"hello"という値の所有権をもっている
    let s2 = s1; //s2に所有者を移動

    //所有権がないのでコンパイルエラーとなる
    //println!("{}", s1);

    println!("{}", s2);

    let s3 = s2.clone();

    //clone()でコピーすることは可能
    println!("{}", s2);
    println!("{}", s3);

    {
        let s4 = String::from("hi");
        println!("{}, world!", s4);
    } //ブロックから外れたs4は破棄され、メモリも開放される

    //これはエラーとなる
    //println!("{}, world!", s4);

    //数値やbool型はclone()なしでコピーされる

    let n1: i32 = 100;
    let n2: i32 = n1;

    println!("{}\n{}", n1, n2);

    
    let n1: f32 = 10.0;
    let n2: f32 = n1;

    println!("{}\n{}", n1, n2);

    let b1: bool = true;
    let b2: bool = b1;

    println!("{}\n{}", b1, b2);


}

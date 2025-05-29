fn main() {
    // 可変配列
    let mut s = String::new();


    std::io::stdin().read_line(&mut s).expect("入力エラー");
    println!();

    let mut num: usize;
    num = s.trim().parse().expect("変換エラー");
    println!("{}", num);

    //クロージャーを使うことで短縮できる
    let scan = |str: &mut String| {
        std::io::stdin().read_line(str).expect("入力エラー");
    };

    // 単語をつなげるときは_でつなげる
    let parse_int =
        |str: &String| -> usize { str.trim().parse::<usize>().expect("変換エラー") 
    };

    let mut s2 = String::new();
    scan(&mut s2);
    let num2 = parse_int(&s2);
    num += num2;
    println!("{}", num);
}

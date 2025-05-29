//定数定義
const SIZE: usize = 10;

fn main() {
    //[初期値;要素数]
    let mut a = [0; SIZE];

    for i in 1..=10 {
        a[i - 1] = i;
    }

    // クロージャー let 名前 = |引数| 定義;
    let double = |x:& usize| *x * 2;

    //iter_mut()は可変可能なイテレータを返す(今回は&mut usizeと解釈される)
    //C言語のようなポインタと拡張for文の組み合わせ技
    for n in a.iter_mut(){
        *n = double(n);
    }

    println!("{:?}", a);
}

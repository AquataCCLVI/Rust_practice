//定数定義
const SIZE: usize = 10;

//fn 関数名(引数)
fn init(array: &mut [usize; SIZE]) {
    for i in 1..=10{
        array[i - 1] = i;
    }
}

//戻り値は -> 型で示す 戻り値を指定するときは「;」不要、return 〇〇;のような方法も使用可能
fn print_array(array: & [usize; SIZE]) -> String{
    let mut str = String::from("");
    for i in 0..SIZE{
        str.push_str(&array[i].to_string());
        str.push_str(", ");
    }

    str
}

fn main() {
    //[初期値;要素数]
    let mut a = [0; SIZE];

    for i in 1..=SIZE {
        a[i - 1] = i;
    }

    //[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    println!("{:?}", a);

    let b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for n in b {
        print!("{}, ", n);
    }

    println!();

    let mut c = [0; SIZE];
    init(&mut c);
    println!("{}", print_array(&c));
}

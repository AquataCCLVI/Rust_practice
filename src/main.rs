fn main() {
    let mut sum = 0;

    for i in 0..100 {
        if i % 2 == 0 {
            println!("{}", i);
        }
        sum += i;
    }
    println!("{}", sum);
}

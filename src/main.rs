fn main() {
    let x = 10;
    let y = 20;

    println!("{}", x + y);
    println!("{}", x - y);
    println!("{}", x * y);
    println!("{}", x / y);

    //型指定しないとi32型として解釈される
    let fx: f32 = 10.0;
    let fy: f32 = 20.0;

    println!("{}", fx + fy);
    println!("{}", fx - fy);
    println!("{}", fx * fy);
    println!("{}", fx / fy);
}

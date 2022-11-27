fn main() {
    let a = 3;
    println!("Hello, world!!");
    {
        let b = a;
    }
    println!("{}", b);
}

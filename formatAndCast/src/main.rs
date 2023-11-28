fn main() {
    println!("Hello, world!");
    let mut x = 10;
    println!("tests{}",x);
    x=100;
    println!("{}",x);
    let x = "test";
    println!("{}tests{}",x,x);
    let y:i32 = 78;
    println!("{}sdfasf",y);

    let decimal: f32 = 22.2;
    let integer: i32 = decimal as i32;

    println!("decimal: {}, integer: {}", decimal, integer)
}
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
    let input = String::from("hello world");
    let max_width = usize::from_be(5);
    let mut writer = BufWriter::new(stdout().lock());
    say(input.as_bytes(), max_width,&mut writer).unwrap();
}

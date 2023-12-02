
fn main() {
    let b:Option<u32> = Some(5);
    let c = b.map(|x| x + 5);
    println!("meow: {}", c.unwrap_or(0));
}

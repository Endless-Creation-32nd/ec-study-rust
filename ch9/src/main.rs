use ch9::Guess::Guess;

extern crate ch9;
fn main() {
    let mut a = Guess::new(99);
    a.set_value(11);
    println!("{}", a.get_value());
}

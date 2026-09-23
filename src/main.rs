use std::env::args;

fn main() {
    let arg = args();
    for a in arg {
        println!("{a}");
    }
}

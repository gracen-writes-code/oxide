use std::fs;

fn main() -> ! {
    for file in fs::read_dir("/").unwrap() {
        println!("{:?}", file);
    }

    loop {}
}
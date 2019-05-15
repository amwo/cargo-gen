fn main() {
    for arg in std::env::args().skip(2) {
        println!("{}", arg);
    }
}

fn main() {
    println!("Current count: {}", server::read_db().unwrap());
}

pub mod days;

fn main() {
    println!("{}", days::dayone::dayone1("inputday1.txt").unwrap());
    println!("{}", days::dayone::dayone2("inputday1.txt").unwrap());
}

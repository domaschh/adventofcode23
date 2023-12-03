pub mod days;

fn main() {
    println!("{}", days::dayone::dayone1("inputday1.txt").unwrap());
    println!("{}", days::dayone::dayone2("inputday1.txt").unwrap());
    println!(
        "{}",
        days::daytwo::daytwo1("inputday2.txt", 12, 14, 13).unwrap()
    );
    println!("{}", days::daytwo::daytwo2("inputday2.txt").unwrap());
}

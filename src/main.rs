pub mod days;

fn main() {
    println!(
        "Day1: 1 {}",
        days::dayone::dayone1("inputday1.txt").unwrap()
    );
    println!(
        "Day1: 2 {}",
        days::dayone::dayone2("inputday1.txt").unwrap()
    );

    println!(
        "Day2: 1 {}",
        days::daytwo::daytwo1("inputday2.txt", 12, 14, 13).unwrap()
    );
    println!(
        "Day2: 2 {}",
        days::daytwo::daytwo2("inputday2.txt").unwrap()
    );

    println!(
        "Day3: 1 {}",
        days::daythree::daythree1("inputday3.txt").unwrap()
    );
    println!(
        "Day3: 2 {}",
        days::daythree::daythree2("inputday3.txt").unwrap()
    );
    println!(
        "Day4: 1 {}",
        days::dayfour::dayfour1("inputday4.txt").unwrap()
    );
    println!(
        "Day4: 2 {}",
        days::dayfour::dayfour2("inputday4.txt").unwrap()
    );

    println!(
        "Day5: 1 {}",
        days::dayfive::dayfive1("inputday5.txt").unwrap()
    );

    // println!(
    //     "Day5: 2 {}",
    //     days::dayfive::dayfive2("inputday5.txt").unwrap()
    // );
    println!(
        "Day6: 1 {}",
        days::daysix::daysix1("inputday6.txt").unwrap()
    );

    println!(
        "Day6: 2 {}",
        days::daysix::daysix2("inputday6.txt").unwrap()
    );
    println!("Day7: 1 {}", days::dayseven::dayseven1("inputday7.txt"));

    println!("Day7: 2 {}", days::dayseven::dayseven2("inputday7.txt"));
    println!(
        "Day8: 1 {}",
        days::dayeight::dayeight1("inputday8.txt").unwrap()
    );

    println!(
        "Day8: 2 {}",
        days::dayeight::dayeight2("inputday8.txt").unwrap()
    );
    println!(
        "Day9: 1 {}",
        days::daynine::daynine1("inputday9.txt").unwrap()
    );

    println!(
        "Day9: 2 {}",
        days::daynine::daynine2("inputday9.txt").unwrap()
    );
    println!(
        "Day10: 1 {}",
        days::dayten::dayten1("inputday10.txt").unwrap()
    );

    println!(
        "Day10: 2 {}",
        days::dayten::dayten2("inputday10.txt").unwrap()
    );
    println!(
        "Day11: 1 {}",
        days::dayeleven::dayeleven1("inputday11.txt").unwrap()
    );

    println!(
        "Day11: 2 {}",
        days::dayeleven::dayeleven1("inputday11.txt").unwrap()
    );
}

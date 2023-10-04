fn main() {
    print_labeled_measurement(5, 'h');
}

fn five() -> u8 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
    println!("{}", five());
    println!("{}", plus_one(5));
}

fn plus_one(x: u8) -> u8 {
    x + 1
}
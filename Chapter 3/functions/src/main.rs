fn main() {
    print_labeled_measurement(5, 'h');
    stat_expression(0);
    println!("{}", five());
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn stat_expression(y: u32) {
    let x = {
        let z = y + 5;
        z + 10
    };
    println!("{}", x);
}

fn five() -> u8 {
    5
}

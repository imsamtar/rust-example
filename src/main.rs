fn main() {
    let chars = ['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd'];
    print_array(chars)
}

fn print_array(array: [char; 11]) {
    for i in array {
        print!("{}", i);
    }
    println!("");
}

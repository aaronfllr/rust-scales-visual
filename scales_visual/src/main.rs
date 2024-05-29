use std::io;

fn main() {
    let whole_notes: [&str; 12] = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];
    println!("Please enter the tuning that you are using: ");
    println!("Please enter the key that you are using: ");
    println!("Please enter the scale that you want to see: ");

    let mut tuning = String::new();
    let mut key = String::new();
    let mut scale = String::new();

    io::stdin()
        .read_line(&mut tuning)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line");

    println!("you entered: {} {} {} {}", tuning, key, scale, whole_notes[0]);
}

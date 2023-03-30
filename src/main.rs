use std::io;

fn main() {

    // Take username from input then remove Newline.
    let mut username_input = String::new();
    println!("Enter your username:");
    io::stdin()
        .read_line(&mut username_input)
        .expect("Failed to read the username.");

    let len = username_input.trim_end_matches(&['\r', '\n'][..]).len();
    username_input.truncate(len);

    // Take number input then convert to u32.
    let mut number_input = String::new();
    println!("Enter a number:");
    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read line.");

    let n: u32 = number_input.trim().parse().expect("Input is not a number");

    let mut z = String::new();

    // Take each character, cast as u32 then add the user number to it then add it to String z.
    for char in username_input.chars() {
        let c = char as u32 + n;
        z.push(char::from_u32(c).unwrap());
    }
    println!("Your password is: {}", z);

    // Stop window from closing until any key is pressed.
    dont_disappear::any_key_to_continue::default();
}

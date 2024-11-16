use std::io;
mod diamond;

fn main() {
    println!("Welcome to Diamond!");

    let letter = loop {
        let mut letter = String::new();
        println!("Select a Letter: ");
        io::stdin()
            .read_line(&mut letter)
            .expect("Failed to read line");

        let trimmed_letter = letter.trim();
        let letter: String = match trimmed_letter.chars().count() {
            1 => {
                trimmed_letter.to_string()
            }
            _ => {
                println!("Please enter a single character");
                continue;
            }
        };
        break letter;
        // Use the letter variable here
    };
    println!("You selected: {}", letter);

    // Draw the diamond
    let diamond = diamond::draw(letter);
    println!("{}", diamond);

}

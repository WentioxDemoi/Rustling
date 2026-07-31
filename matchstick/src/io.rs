use std::io::{self, Write};

// Pour récup les inputs du user
pub fn get_line() -> Option<String> {
    let mut line = String::new();

    let result = io::stdin().read_line(&mut line);
    match result {
        Ok(_) => {
            return Some(line.trim().to_string());
        }
        Err(error) => {
            display_msg(error.to_string());
            None
        }
    }
}

// Pour afficher l'arbre
pub fn display_tab(tab: &Vec<u8>) {
    tab.iter().enumerate().for_each(|(i, nb)| println!("[{}]{}{}", i, " ".repeat(tab.len() - 1 - i), "|".repeat(*nb as usize))); 
}

// Pour afficher les messages ingame
pub fn display_msg(msg: String)
{
    println!("{}", msg);
}

// Fonction qui sert à clear le terminal
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
use crate::io;
use crate::Tab;

pub fn engine(tab: Tab) {
    let ingame: bool = true;
    while ingame {
        io::display_tab(tab.get_tab());
        io::display_msg("Which line do you want to target ?\nLine number : ".to_string());
        io::get_line();
    }
    
}
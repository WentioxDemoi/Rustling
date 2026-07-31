use crate::Tab;
use crate::io;

use std::num::ParseIntError;
// use rand::Rng;
use rand::RngExt;
// use rand::random_bool;
// use rand::random_range;

pub struct Engine {
    tab: Tab,
    nb_sticks_allowed: u8,
}

impl Engine {
    pub fn new(tab: Tab, nb_sticks_allowed: u8) -> Self {
        Engine {
            tab: tab,
            nb_sticks_allowed: nb_sticks_allowed,
        }
    }

    fn get_target(&self) -> u8 {
        io::display_tab(self.tab.get_tab());
        io::display_msg("Which line do you want to target ?\nLine number : ".to_string());
        if let Some(target) = io::get_line() {
            let line: Result<i64, ParseIntError> = target.parse();
            match line {
                Ok(nb) => {
                    if (nb >= 0 && (nb as usize) < self.tab.get_tab().len())
                        && self.tab.get_nb_sticks_on_line(nb as usize) > 0
                    {
                        io::clear_screen();
                        return nb as u8;
                    } else {
                        io::clear_screen();
                        io::display_msg("Line not in range !".to_string());
                        self.get_target()
                    }
                }
                Err(_) => {
                    io::clear_screen();
                    io::display_msg("Cannot parse input !".to_string());
                    self.get_target()
                }
            }
        } else {
            io::clear_screen();
            self.get_target()
        }
    }

    fn get_nb_sticks_to_remove(&self, line: u8) -> u8 {
        io::display_tab(self.tab.get_tab());
        io::display_msg("How many sticks do you want to remove ?\nStick number : ".to_string());
        if let Some(target) = io::get_line() {
            let nb_sticks: Result<i64, ParseIntError> = target.parse();
            match nb_sticks {
                Ok(nb) => {
                    if nb > 0 && nb <= (self.nb_sticks_allowed as i64) {
                        if nb > (self.tab.get_nb_sticks_on_line(line as usize) as i64) {
                            io::clear_screen();
                            io::display_msg("Not enough sticks on the line ?".to_string());
                            self.get_nb_sticks_to_remove(line)
                        } else {
                            io::clear_screen();
                            return nb as u8;
                        }
                    } else {
                        io::clear_screen();
                        io::display_msg("nb of sticks not in range !".to_string());
                        self.get_nb_sticks_to_remove(line)
                    }
                }
                Err(_) => {
                    io::clear_screen();
                    io::display_msg("Cannot parse input !".to_string());
                    self.get_nb_sticks_to_remove(line)
                }
            }
        } else {
            io::clear_screen();
            self.get_nb_sticks_to_remove(line)
        }
    }

    pub fn engine(&mut self) {
        let mut my_turn: bool = true;
        let mut rng = rand::rng();

        loop {
            let line = self.get_target();
            let nb = self.get_nb_sticks_to_remove(line);
            self.tab.remove(line as usize, nb as usize);
            if self.tab.get_total_nb_sticks_left() == 0 {
                break;
            }
            my_turn = !my_turn;
            let mut random_line: usize;
            loop {
                random_line = rng.random_range(0..self.tab.get_tab().len());
                if self.tab.get_nb_sticks_on_line(random_line) > 0 {
                    break;
                }
            }
            
            let random_nb: u8;
            if self.tab.get_nb_sticks_on_line(random_line) > self.nb_sticks_allowed {
                random_nb = rng.random_range(1..=self.nb_sticks_allowed);
            } else {
                random_nb = rng.random_range(1..=self.tab.get_nb_sticks_on_line(random_line));
            }
            self.tab.remove(random_line, random_nb as usize);
            io::display_msg(
                "AI removed ".to_string()
                    + &random_nb.to_string()
                    + " sticks from line "
                    + &random_line.to_string()
                    + " !",
            );
            if self.tab.get_total_nb_sticks_left() == 0 {
                break;
            }
            my_turn = !my_turn;
        }

        if my_turn {
            io::display_msg("You lost !".to_string());
        } else {
            io::display_msg("AI lost !".to_string());
        }
    }
}

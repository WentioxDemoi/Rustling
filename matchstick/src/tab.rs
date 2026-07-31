pub struct Tab {
    tab: Vec<String>,
}

impl Tab {

    pub fn new() -> Self {
        Tab { tab: Vec::new() }
    }
    pub fn remove(&mut self, line_nb: usize, nb: usize) {
    if let Some(line) = self.tab.get_mut(line_nb) {
        *line = line.replacen('|', " ", nb);
    }
}

    pub fn init(&mut self, nb_lines: usize) {
        self.tab = (0..nb_lines).map(|index| { 
            let mut str: String = String::new();
            str += &" ".repeat(nb_lines - 1 - index);  
            str += &"|".repeat(1 + (2 * index)); 
            str += &" ".repeat(nb_lines - 1 - index);
            str
        }
        ).collect();
    }

    pub fn get_tab(&self) -> &Vec<String> {
        &self.tab
    }

    pub fn get_nb_sticks_on_line(&self, line: usize) -> u8 {
        self.tab.get(line).unwrap().matches('|').count() as u8 // On peut se le permettre car on est sur que le return sera < à 255
    }

    pub fn get_total_nb_sticks_left(&self) -> u8 {
        let mut count = 0;
        self.tab.iter().for_each(|string| { count += string.matches('|').count() as u8; });
        count

    }
}
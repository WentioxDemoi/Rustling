pub struct Tab {
    tab: Vec<u8>,
}

impl Tab {

    pub fn new() -> Self {
        Tab { tab: Vec::new() }
    }
    pub fn remove(&mut self, line_nb: usize, nb: u8) {
        if let Some(line) = self.tab.get_mut(line_nb) {
            *line -= nb;
        }
}

    pub fn init(&mut self, nb_lines: u8) {
        self.tab = (0..nb_lines).map(|index| { 
            let nb_sticks = index * 2 + 1;
            nb_sticks
        }
        ).collect();
    }

    pub fn get_tab(&self) -> &Vec<u8> {
        &self.tab
    }

    pub fn get_nb_sticks_on_line(&self, line: usize) -> u8 {
        *self.tab.get(line).unwrap() // On peut se le permettre car on est sur que le return sera < à 255
    }

    pub fn get_total_nb_sticks_left(&self) -> u8 {
        let mut count = 0;
        self.tab.iter().for_each(|nb| { count += nb; });
        count

    }
}
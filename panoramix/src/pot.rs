pub struct Pot {
    pot_size: u8,
    nb_serving_left: u8,
    panoramix_pinged: bool,
    no_more: bool,
}

impl Pot {
    pub fn new(
        pot_size_: u8,
        nb_serving_left_: u8,
        panoramix_pinged_: bool,
        no_more_: bool,
    ) -> Self {
        Pot {
            pot_size: pot_size_,
            nb_serving_left: nb_serving_left_,
            panoramix_pinged: panoramix_pinged_,
            no_more: no_more_,
        }
    }

    pub fn refill(&mut self) {
        self.nb_serving_left = self.pot_size;
    }

    pub fn remove_serving(&mut self) {
        self.nb_serving_left -= 1;
    }

    pub fn get_serving_left(&self) -> u8 {
        self.nb_serving_left
    }

    pub fn get_no_more(&self) -> bool {
        self.no_more
    }

    pub fn set_no_more(&mut self, value: bool) {
        self.no_more = value;
    }

    pub fn get_panoramix_pinged(&self) -> bool {
        self.panoramix_pinged
    }

    pub fn set_panoramix_pinged(&mut self, value: bool) {
        self.panoramix_pinged = value;
    }
}

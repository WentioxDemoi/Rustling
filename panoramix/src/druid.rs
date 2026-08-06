use crate::pot::Pot;
use crate::message::Message;
use std::sync::{Arc, Mutex, mpsc};

pub struct Druid {
    nb_refill_left: u8,
    villagers_endpoints: Vec<mpsc::Sender<Message>>,
    listenner: mpsc::Receiver<Message>,
    pot: Arc<Mutex<Pot>>,
}

impl Druid {
    pub fn new(
        nb_refill_left_: u8,
        villagers_endpoints_: Vec<mpsc::Sender<Message>>,
        listenner_: mpsc::Receiver<Message>,
        pot_: Arc<Mutex<Pot>>,
    ) -> Self {
        Self {
            nb_refill_left: nb_refill_left_,
            villagers_endpoints: villagers_endpoints_,
            listenner: listenner_,
            pot: pot_,
        }
    }

    pub fn druid_start(&mut self) {
        println!("Druid: I'm ready... but sleepy...");

        let mut nb_active_villagers = self.villagers_endpoints.len();

        while nb_active_villagers > 0 && self.nb_refill_left > 0 {
            match self.listenner.recv() {
                Ok(Message::PotEmpty) => self.druid_refill(),
                Ok(Message::Done) => nb_active_villagers -= 1,
                Ok(Message::PotFilled) => {}
                Err(_) => break,
            }
        }

        if self.nb_refill_left == 0 {
            self.druid_out_of_ingredients();
        }
    }

    fn druid_refill(&mut self) {
        self.nb_refill_left -= 1;
        println!(
            "Druid: Ah! Yes, yes, I'm awake! Working on it! Beware I can only make {} more refills after this one.",
            self.nb_refill_left
        );
        {
            let mut pot = self.pot.lock().unwrap();
            pot.refill();
            pot.set_panoramix_pinged(false);
        }
        self.villagers_endpoints
            .retain(|tx| tx.send(Message::PotFilled).is_ok());
    }

    fn druid_out_of_ingredients(&self) {
        self.pot.lock().unwrap().set_no_more(true);
        println!("Druid: I'm out of viscum. I'm going back to... zZz");
    }
}

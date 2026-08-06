use crate::pot::Pot;
use crate::message::Message;
use std::sync::{Arc, Mutex, mpsc};

enum PotAction {
    Drink,
    CallDruidAndWait,
    Wait,
    Sleep,
}

pub struct Villager {
    nb_fights_left: u8,
    pot: Arc<Mutex<Pot>>,
    druid_endpoints: mpsc::Sender<Message>,
    listenner: mpsc::Receiver<Message>,
}

impl Villager {
    pub fn new(
        nb_fights_left_: u8,
        pot_: Arc<Mutex<Pot>>,
        druid_endpoints_: mpsc::Sender<Message>,
        listenner_: mpsc::Receiver<Message>,
    ) -> Self {
        Self {
            nb_fights_left: nb_fights_left_,
            pot: pot_,
            druid_endpoints: druid_endpoints_,
            listenner: listenner_,
        }
    }

    pub fn villager_start(&mut self, id: u8) {
        println!("Villager {}: Going into battle!", id);

        loop {
            if self.nb_fights_left == 0 {
                self.villager_sleep(id);
                break;
            }

            match self.check_pot(id) {
                PotAction::Drink => {
                    self.nb_fights_left -= 1;
                    self.villager_fight(id, self.nb_fights_left);
                }
                PotAction::CallDruidAndWait => {
                    self.villager_call_druid(id);
                    let _ = self.listenner.recv();
                }
                PotAction::Wait => {
                    let _ = self.listenner.recv();
                }
                PotAction::Sleep => {
                    self.villager_sleep(id);
                    break;
                }
            }
        }
    }

    fn check_pot(&self, id: u8) -> PotAction {
        let mut pot = self.pot.lock().unwrap();
        let serving_left = pot.get_serving_left();

        println!(
            "Villager {}: I need a drink... I see {} servings left.",
            id, serving_left
        );

        if serving_left > 0 {
            pot.remove_serving();
            return PotAction::Drink;
        }

        if pot.get_no_more() {
            return PotAction::Sleep;
        }

        if !pot.get_panoramix_pinged() {
            pot.set_panoramix_pinged(true);
            let _ = self.druid_endpoints.send(Message::PotEmpty);
            return PotAction::CallDruidAndWait;
        }

        PotAction::Wait
    }

    fn villager_call_druid(&self, id: u8) {
        println!("Villager {}: Hey Pano wake up! We need more potion.", id);
    }

    fn villager_fight(&self, id: u8, fights_left: u8) {
        println!(
            "Villager {}: Take that roman scum! Only {} left.",
            id, fights_left
        );
    }

    fn villager_sleep(&self, id: u8) {
        let _ = self.druid_endpoints.send(Message::Done);
        println!("Villager {}: I'm going to sleep now.", id);
    }
}

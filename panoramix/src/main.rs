use std::{
    env,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

mod druid;
mod pot;
mod villager;
mod message;

// Dans ce projet il faudra utiliser des Arc<Mutex>> pour partager l'accès aux ressources (pot) entre les Villagers mais aussi avec le Druid
// Il faudra aussi utiliser MPSC pour communiquer entre le Druid et les Villagers

fn print_help(program_name: &str) {
    eprintln!(
        "USAGE: {} <nb_villagers> <pot_size> <nb_fights> <nb_refills>",
        program_name
    );
    eprintln!();
    eprintln!("\tnb_villagers: indicates the number of villagers (must be >0);");
    eprintln!(
        "\tpot_size: indicates the maximum number that can be contained in the cooking pot (must be >0);"
    );
    eprintln!(
        "\tnb_fights: indicates the maximum number of fights a villager will engage in (must be >0);"
    );
    eprintln!(
        "\tnb_refills: indicates the maximum number of time the druid will refill the pot (must be >0)."
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_help(&args[0]);
        return Ok(());
    }
    let pot_size_ = args.get(2).unwrap().parse::<u8>()?;
    let nb_refill_left_ = args.get(4).unwrap().parse::<u8>()?;
    let nb_villagers_ = args.get(1).unwrap().parse::<u8>()?;
    let nb_fights_ = args.get(3).unwrap().parse::<u8>()?;

    if pot_size_ < 1 || nb_refill_left_ < 1 || nb_villagers_ < 1 || nb_fights_ < 1 {
        print_help(&args[0]);
        return Err("Invalid arguments".into());
    }

    let (tx_villager_to_druid, rx_villager_to_druid) = mpsc::channel();

    let mut villagers_endpoints_ = Vec::new();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let pot_ = Arc::new(Mutex::new(pot::Pot::new(
        pot_size_, pot_size_, false, false,
    )));

    for i in 0..nb_villagers_ {
        let (tx_druid_to_villager, rx_druid_to_villager) = mpsc::channel();

        villagers_endpoints_.push(tx_druid_to_villager);

        let mut instance = villager::Villager::new(
            nb_fights_,
            pot_.clone(),
            tx_villager_to_druid.clone(),
            rx_druid_to_villager,
        );

        handles.push(thread::spawn(move || {
            instance.villager_start(i);
        }));
    }

    let mut druid_ = druid::Druid::new(
        nb_refill_left_,
        villagers_endpoints_,
        rx_villager_to_druid,
        pot_.clone(),
    );

    handles.push(thread::spawn(move || {
        druid_.druid_start();
    }));

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

use std::env;

fn print_help(program_name: &str) {
    eprintln!("USAGE: {} <nb_villagers> <pot_size> <nb_fights> <nb_refills>", program_name);
    eprintln!();
    eprintln!("\tnb_villagers: indicates the number of villagers (must be >0);");
    eprintln!("\tpot_size: indicates the maximum number that can be contained in the cooking pot (must be >0);");
    eprintln!("\tnb_fights: indicates the maximum number of fights a villager will engage in (must be >0);");
    eprintln!("\tnb_refills: indicates the maximum number of time the druid will refill the pot (must be >0).");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_help(&args[0]);
        return;
    }
    

}

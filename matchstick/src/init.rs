
#[derive(Debug)]
pub enum StartingArgsError {
    NotTheGoodNbOfArgs,
    NotTheGoodNbOfLines,
    NotTheGoodNbOfSticksAllowed,

    // Ici on évite d'écrire ParseInt(ParseIntError) car on s'en fou de récup l'erreur émise par parse().
    // C'est uiquement pour éviter le warning et car on sait que si parse renvoie une erreur, c'est que l'utilisateur n'a pas bien écrit son nombre
    ParseInt,
    Help
}

fn print_usage() {
    eprintln!("USAGE: ./matchstick n m");
    eprintln!("\tn\tnumber of lines (1 < n < 100)");
    eprintln!("\tm\tmax number of matches per turn (m > 0)");
}

pub fn errors_cases(args: Vec<String>) -> Result<(u8, u8), StartingArgsError> {
    print_args(&args);
    let nb_lines: u64;
    let nb_sticks_allowed: u64;

    if args.len() != 3 {
        if args.get(1) == Some(&"-h".to_string()) {
            print_usage();
            return Err(StartingArgsError::Help);
        }
        return Err(StartingArgsError::NotTheGoodNbOfArgs);
    }

    match args.get(1) {
        Some(nb_str) => {
            nb_lines = nb_str.parse().map_err(|_| StartingArgsError::ParseInt)?; // Ici on fait un lambda qui jette l'erreur ParseIntError pour return notre erreur
            if nb_lines < 3 || nb_lines > 100 {
                return Err(StartingArgsError::NotTheGoodNbOfLines);
            }
        }
        _ => {
            print_usage();
            return Err(StartingArgsError::NotTheGoodNbOfArgs); // Ne devrait pas arriver étant donné la première sécu (Obligation du compilateur)
        }
    }

    match args.get(2) {
        Some(nb_str) => {
            nb_sticks_allowed = nb_str.parse().map_err(|_| StartingArgsError::ParseInt)?;
            if nb_sticks_allowed <= 0 || nb_sticks_allowed > 255 {
                return Err(StartingArgsError::NotTheGoodNbOfSticksAllowed);
            }
        }
        _ => {
            print_usage();
            return Err(StartingArgsError::NotTheGoodNbOfArgs); // Ne devrait pas arriver étant donné la première sécu (Obligation du compilateur)
        }
    }
    Ok((nb_lines as u8, nb_sticks_allowed as u8))
}

fn print_args(args: &Vec<String>) {
    args.iter()
        .for_each(|arg| println!("Arg starting block : {}", arg));
}

mod test;
use std::env::args;

#[derive(Debug)]
enum StartingArgsError {
    NotTheGoodNbOfArgs,
    NotTheGoodNbOfLines,
    NotTheGoodNbOfSticksAllowed,
    
    // Ici on évite d'écrire ParseInt(ParseIntError) car on s'en fou de récup l'erreur émise par parse().
    // C'est uiquement pour éviter le warning et car on sait que si parse renvoie une erreur, c'est que l'utilisateur n'a pas bien écrit son nombre
    ParseInt 
}



fn errors_cases(args: Vec<String>) -> Result<(), StartingArgsError> {
    if args.len() != 3 {
        return Err(StartingArgsError::NotTheGoodNbOfArgs)
    }

    let nb_lines = args.get(1);
    match nb_lines {
        Some(nb_str) => {
            let nb: u64 = nb_str.parse().map_err(|_| StartingArgsError::ParseInt)?; // Ici on fait un lambda qui jette l'erreur ParseIntError pour return notre erreur
            if nb < 3 || nb > 100 { return Err(StartingArgsError::NotTheGoodNbOfLines) }
        }
        _ => return Err(StartingArgsError::NotTheGoodNbOfArgs) // Ne devrait pas arriver étant donné la première sécu (Obligation du compilateur)
    }

    let nb_sticks_allowed = args.get(2);
    match nb_sticks_allowed {
        Some(nb_str) => {
            let nb: u64 = nb_str.parse().map_err(|_| StartingArgsError::ParseInt)?;
            if nb <= 0 { return Err(StartingArgsError::NotTheGoodNbOfSticksAllowed) }
        }
        _ => return Err(StartingArgsError::NotTheGoodNbOfArgs) // Ne devrait pas arriver étant donné la première sécu (Obligation du compilateur)
    }
    Ok(())
}

fn print_args(args: &Vec<String>) {
    args.iter().for_each(
        |arg| println!("Arg starting block : {}", arg));
}

fn main() -> Result<(), StartingArgsError> {
    let args: Vec<String> = args().collect();
    print_args(&args);
    errors_cases(args)?;
    println!("Hello, world! {}", test::slt());
    Ok(())
}

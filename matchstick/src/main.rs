mod init;
mod io;
mod engine;
mod tab;
use std::env::args;

use crate::tab::Tab;




fn main() -> Result<(), init::StartingArgsError> {
    let args: Vec<String> = args().collect();
    let (nb_lines, nb_sticks_allowed) = init::errors_cases(args)?;
    let mut tab = Tab::new();
    tab.init(nb_lines as usize);
    engine::engine(tab);

    Ok(())
}


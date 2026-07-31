mod init;
mod io;
mod engine;
mod tab;
use std::env::args;

use crate::tab::Tab;
use crate::engine::Engine;




fn main() -> Result<(), init::StartingArgsError> {
    let args: Vec<String> = args().collect();
    let (nb_lines, nb_sticks_allowed) = init::errors_cases(args)?;
    let mut tab = Tab::new();
    tab.init(nb_lines as usize);
    let mut engine = Engine::new(tab, nb_sticks_allowed);
    engine.engine();
    Ok(())
}


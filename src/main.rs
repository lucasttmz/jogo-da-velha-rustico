use std::{env, process};

use jogo_da_velha_rustico::{Partida, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    Partida::new().jogar();
}

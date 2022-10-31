use crate::game::Arkanoid;
use crate::main_loop::run;
use std::error::Error;

mod assets;
mod ball;
mod game;
mod main_loop;
mod renderer;
mod renderer_ext;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    run::<Arkanoid>();
    Ok(())
}

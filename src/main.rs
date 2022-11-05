use crate::game::Arkanoid;
use crate::main_loop::run;
use std::error::Error;

mod assets;
mod ball;
mod collisions;
mod game;
mod input;
mod main_loop;
mod palette;
mod renderer;
mod renderer_ext;
mod block;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    run::<Arkanoid>();
    Ok(())
}

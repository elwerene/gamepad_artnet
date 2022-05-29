mod artnet;
mod gamepad;
mod opts;

use env_logger::Env;

pub use opts::OPTS;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let artnet_sender = artnet::start();
    gamepad::listen(artnet_sender);
}

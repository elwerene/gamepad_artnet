use clap::Parser;
use once_cell::sync::Lazy;

pub static OPTS: Lazy<Opts> = Lazy::new(Opts::parse);

#[derive(Parser, Debug)]
pub struct Opts {
    // Host which should receive artnet commands
    pub artnet_host: String,

    // Artnet universe
    pub artnet_universe: u16,
}

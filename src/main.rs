use crate::config::Config::Config;

mod args;
mod config;
mod debug;
mod discord;

fn main() {
    let conf = Config::new();
}

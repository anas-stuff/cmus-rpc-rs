use crate::config::config::Config;

mod args;
mod config;
mod debug;
mod discord;

fn main() {
    let _conf = Config::new();
}

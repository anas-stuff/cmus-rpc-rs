use crate::config::config::Config;
use crate::discord::DiscordController::DiscordController;

mod args;
mod config;
mod debug;
mod discord;
mod cmus;

fn main() {
    let _conf = Config::new();
    DiscordController::new(961407969986232380, debug::debugger::Debugger::new());
    std::thread::sleep(std::time::Duration::from_secs(20));
}

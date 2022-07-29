use crate::config::config::Config;
use crate::discord::discord_controller::DiscordController;

mod args;
mod cmus;
mod config;
mod debug;
mod discord;

fn main() {
    let conf = Config::new();
    let mut debugger = debug::debugger::Debugger::new();
    debugger.set_debug(conf.debug);

    let mut discord_controller = DiscordController::new("961407969986232380", &debugger);
    cmus::tracker::run(&conf, &debugger, &mut discord_controller);
}

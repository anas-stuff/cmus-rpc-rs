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

    let mut sleep_time_arc = std::sync::Arc::new(std::sync::Mutex::new(0u32));

    let time = std::sync::Arc::clone(&sleep_time_arc);
    std::thread::spawn(move || {
        // Increment sleep_time_mutex every second
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let mut sleep_time = time.lock().unwrap();
            *sleep_time += 1;
        }
    });

    let mut discord_controller = DiscordController::new("961407969986232380", &debugger);
    cmus::tracker::run(&conf, &debugger, &mut discord_controller, &mut sleep_time_arc);
}

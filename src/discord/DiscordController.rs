use crate::debug;
use crate::debug::Debugger::Debugger;

pub struct DiscordController {
    pub start_time: i32,
    drpc: discord_presence::Client,
}

impl DiscordController {
    pub fn new(app_id: u64, debugger: Debugger) -> DiscordController {
        let mut controller = DiscordController {
            start_time: -1,
            drpc: discord_presence::Client::new(app_id),
        };
        controller.drpc.on_ready(move |_| {
            debugger.log("cmus rpc ready");
        });
        controller.drpc.start();
        controller
    }
}

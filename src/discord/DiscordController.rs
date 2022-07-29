use discord_presence::models::Activity;
use crate::cmus;
use crate::debug::debugger::Debugger;

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

    pub fn update_presence(&mut self, cmus_response: cmus::Response) {
        self.drpc.update_presence(activity);
    }
}

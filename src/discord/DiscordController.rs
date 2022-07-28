use crate::debug::debugger::Debugger;

pub struct discord_controller {
    pub start_time: i32,
    drpc: discord_presence::Client,
}

impl discord_controller {
    pub fn new(app_id: u64, debugger: Debugger) -> discord_controller {
        let mut controller = discord_controller {
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

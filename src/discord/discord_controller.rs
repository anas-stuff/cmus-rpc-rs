use crate::cmus;
use crate::config::config;
use crate::debug::debugger::Debugger;
use crate::discord::formatter;

pub struct DiscordController {
    pub start_time: i32,
    drpc: discord_rpc_client::Client,
}

impl DiscordController {
    pub fn new(app_id: u64, debugger: &Debugger) -> DiscordController {
        let mut controller = DiscordController {
            start_time: -1,
            drpc: discord_rpc_client::Client::new(app_id),
        };

        controller.drpc.start();

        debugger.log("Discord RPC client started");

        controller
    }

    pub fn update_presence(&mut self, cmus_response: cmus::responce::Response,
                           debugger: &Debugger,
                           configs: &config::Config) {

        debugger.log("Updating presence");

        let part_1 = formatter::format(configs.part_one_format.as_str(), &cmus_response);
        debugger.log(format!("part_1: {}", part_1).as_str());
        let part_2 = formatter::format(configs.part_two_format.as_str(), &cmus_response);
        debugger.log(format!("part_2: {}", part_2).as_str());

        let activity = discord_rpc_client::models::Activity::new()
            .state(part_2)
            .details(part_1)
            .assets(|assets| {
                assets.large_image(&configs.large_image)
                    .small_image(match cmus_response.state {
                        cmus::responce::State::PLAYING => &configs.playing_image,
                        _ => &configs.paused_image,
                    })
            });
        self.drpc.set_activity(move |_| activity).expect("TODO: panic message");
    }
}

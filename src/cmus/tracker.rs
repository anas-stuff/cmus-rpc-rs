use crate::cmus::responce::Response;
use crate::debug::debugger;
use crate::{discord::discord_controller, Config};

pub fn run(
    configs: &Config,
    debugger: &debugger::Debugger,
    discord_controller: &mut discord_controller::DiscordController,
) {
    let mut buttons_vec = Vec::new();

    if configs.has_button_one() {
        let (label, url) = &configs.button_one;
        buttons_vec.push(discord_rich_presence::activity::Button::new(
            label.as_str(),
            url.as_str(),
        ));
    }
    if configs.has_button_two() {
        let (label, url) = &configs.button_two;
        buttons_vec.push(discord_rich_presence::activity::Button::new(
            label.as_str(),
            url.as_str(),
        ));
    }

    loop {
        let out = std::process::Command::new("cmus-remote")
            .arg("-Q")
            .output()
            .expect("Failed to execute cmus-remote");

        if out.status.code().unwrap() != 0 {
            debugger.log("cmus-remote failed");
            if configs.link {
                std::process::exit(0);
            }
        }

        let cmus_response = String::from_utf8_lossy(&out.stdout).to_string();
        debugger.log(&cmus_response);
        discord_controller.update_presence(
            Response::new(cmus_response),
            debugger,
            configs,
            &buttons_vec,
        );
        debugger.log("Updated presence");
        std::thread::sleep(std::time::Duration::from_secs(configs.interval as u64));
        debugger.log(format!("Sleeping for {} seconds.", &configs.interval.to_string()).as_str());
    }
}

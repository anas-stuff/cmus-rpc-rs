use crate::args;
use crate::config::config_file;

pub struct Config {
    pub debug: bool,
    pub link: bool,
    pub config_path: String,
    pub interval: u32,
    pub sleep: u32,
    pub part_one_format: String,
    pub part_two_format: String,
    pub large_image: String,
    pub playing_image: String,
    pub paused_image: String,
}

impl Config {
    pub fn new() -> Config {
        let args_matches = args::get_args_app().get_matches();
        let parsed_config = match config_file::load() {
            Ok(config) => config,
            Err(e) => {
                println!("Error loading config_path file: {}", e);
                println!("Creating the default config file.");
                config_file::create_default()
            }
        };
        let configs = Config {
            debug: args_matches.is_present("debug") || parsed_config.debug,
            link: args_matches.is_present("link") || parsed_config.link,
            config_path: if args_matches.is_present("config_path") {
                args_matches.value_of("config_path").unwrap().to_string()
            } else {
                parsed_config.config_path
            },
            interval: if args_matches.is_present("interval") {
                args_matches
                    .value_of("interval")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                parsed_config.interval
            },
            sleep: if args_matches.is_present("sleep") {
                args_matches
                    .value_of("sleep")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                parsed_config.sleep
            },
            part_one_format: if args_matches.is_present("part_one_format") {
                args_matches
                    .value_of("part_one_format")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.part_one_format
            },
            part_two_format: if args_matches.is_present("part_two_format") {
                args_matches
                    .value_of("part_two_format")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.part_two_format
            },
            large_image: if args_matches.is_present("large_image") {
                args_matches
                    .value_of("large_image")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.large_image
            },
            playing_image: if args_matches.is_present("playing_image") {
                args_matches
                    .value_of("playing_image")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.playing_image
            },
            paused_image: if args_matches.is_present("paused_image") {
                args_matches
                    .value_of("paused_image")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.paused_image
            },
        };

        configs
    }

    pub fn default() -> Config {
        let config_path = format!(
            "{}/{}",
            dirs::config_dir().unwrap_or_default().to_str().unwrap(),
            "cmus-rps-rs/Config.conf"
        );
        Config {
            debug: false,
            link: false,
            config_path: config_path.to_string(),
            interval: 5, // seconds
            sleep: 5 * 60, // 5 minutes
            part_one_format: "%artist% - %title%".to_string(),
            part_two_format: "%album%".to_string(),
            large_image: "cmus".to_string(),
            playing_image: "play_icon_2".to_string(),
            paused_image: "pause_icon_2".to_string(),
        }
    }
}

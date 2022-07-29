use crate::config::config::Config;

pub(crate) fn new(args_matches: clap::ArgMatches) -> Config {
    let mut config: Config = Config::default();
    Config {
        debug: args_matches.is_present("debug"),
        link: args_matches.is_present("link"),
        config_path: args_matches
            .value_of("config_path")
            .unwrap_or(config.config_path.as_str())
            .to_string(),
        interval: args_matches
            .value_of("interval")
            .unwrap_or(config.interval.to_string().as_str())
            .parse::<u32>()
            .unwrap(),
        sleep: args_matches
            .value_of("sleep")
            .unwrap_or(config.sleep.to_string().as_str())
            .parse::<u32>()
            .unwrap(),
        part_one_format: args_matches
            .value_of("part_one_format")
            .unwrap_or(config.part_one_format.as_str())
            .to_string(),
        part_two_format: args_matches
            .value_of("part_two_format")
            .unwrap_or(config.part_two_format.as_str())
            .to_string(),
        large_image: args_matches
            .value_of("large_image")
            .unwrap_or(config.large_image.as_str())
            .to_string(),
        playing_image: args_matches
            .value_of("playing_image")
            .unwrap_or(config.playing_image.as_str())
            .to_string(),
        paused_image: args_matches
            .value_of("paused_image")
            .unwrap_or(config.paused_image.as_str())
            .to_string(),
        large_text: args_matches
            .value_of("large_text")
            .unwrap_or(config.large_text.as_str())
            .to_string(),
        playing_text: args_matches
            .value_of("playing_text")
            .unwrap_or(config.playing_text.as_str())
            .to_string(),
        paused_text: args_matches
            .value_of("paused_text")
            .unwrap_or(config.paused_text.as_str())
            .to_string(),
        button_one: (
            args_matches
                .value_of("button_one_text")
                .unwrap_or(config.button_one.0.as_str())
                .parse()
                .unwrap(),
            args_matches
                .value_of("button_one_url")
                .unwrap_or(config.button_one.1.as_str())
                .parse()
                .unwrap(),
        ),
        button_two: (
            args_matches
                .value_of("button_two_text")
                .unwrap_or(config.button_two.0.as_str())
                .parse()
                .unwrap(),
            args_matches
                .value_of("button_two_url")
                .unwrap_or(config.button_two.1.as_str())
                .parse()
                .unwrap(),
        ),
    }
}

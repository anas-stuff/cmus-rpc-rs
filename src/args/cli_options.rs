use crate::config::config::Config;

pub(crate) fn new(args_matches: clap::ArgMatches) -> Config {
    Config {
        debug: args_matches.is_present("debug"),
        link: args_matches.is_present("link"),
        config_path: args_matches
            .value_of("config_path")
            .unwrap_or(
                dirs::config_dir()
                    .unwrap_or_default()
                    .as_path()
                    .to_str()
                    .unwrap(),
            )
            .to_string(),
        interval: args_matches
            .value_of("interval")
            .unwrap_or("1000")
            .parse::<u32>()
            .unwrap(),
        sleep: args_matches
            .value_of("sleep")
            .unwrap_or("5000")
            .parse::<u32>()
            .unwrap(),
        part_one_format: args_matches
            .value_of("part_one_format")
            .unwrap_or("{artist} - {title}")
            .to_string(),
        part_two_format: args_matches
            .value_of("part_two_format")
            .unwrap_or("{album}")
            .to_string(),
        large_image: args_matches
            .value_of("large_image")
            .unwrap_or("cmus")
            .to_string(),
        playing_image: args_matches
            .value_of("playing_image")
            .unwrap_or("play_icon_2")
            .to_string(),
        paused_image: args_matches
            .value_of("paused_image")
            .unwrap_or("pause_icon_2")
            .to_string(),
    }
}

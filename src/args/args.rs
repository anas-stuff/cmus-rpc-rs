use clap::{App, Arg};

pub fn get_args_app() -> App<'static> {
    App::new("cmus-rpc-rs")
        .version("0.1.0")
        .author("Anas Elgarhy <anas.elgarhy.dev@gmail.com>")
        .about("A Discord Rich Presence for cmus player")
        .args(&[
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Enable debug mode")
                .takes_value(false),
            Arg::with_name("link")
                .long("link")
                .short('l')
                .help("Linking with cmus (close the program if cmus is not running)")
                .takes_value(false),
            Arg::with_name("config_path")
                .long("config_path")
                .short('c')
                .help("Set custom path to config_path file")
                .takes_value(true),
            Arg::with_name("interval")
                .long("interval")
                .short('i')
                .help("Set custom interval for updating the presence")
                .takes_value(true),
            Arg::with_name("sleep")
                .long("sleep")
                .short('s')
                .help("Set sleep when there is no activity")
                .takes_value(true),
            Arg::with_name("part_one_format")
                .long("partOneFormat")
                .alias("p1f")
                .help("Set custom format for part one of presence")
                .takes_value(true),
            Arg::with_name("part_two_format")
                .long("partTwoFormat")
                .alias("p2f")
                .help("Set custom format for part two of presence")
                .takes_value(true),
            Arg::with_name("large_image")
                .long("largeImage")
                .alias("li")
                .help("Set custom large image for presence")
                .takes_value(true),
            Arg::with_name("playing_image")
                .long("playingImage")
                .alias("pi")
                .help("Set custom playing image for presence")
                .takes_value(true),
            Arg::with_name("paused_image")
                .long("pausedImage")
                .alias("pai")
                .help("Set custom paused image for presence")
                .takes_value(true),
            Arg::with_name("large_text")
                .long("largeText")
                .alias("lt")
                .help("Set custom large text for presence")
                .takes_value(true),
            Arg::with_name("playing_text")
                .long("playingText")
                .alias("pt")
                .help("Set custom playing text for presence")
                .takes_value(true),
            Arg::with_name("paused_text")
                .long("pausedText")
                .alias("pat")
                .help("Set custom paused text for presence")
                .takes_value(true),
        ])
}

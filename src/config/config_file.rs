use crate::config::config::Config;
use std::io::Write;

pub fn load() -> std::io::Result<Config> {
    let mut config: Config = Config::default();
    let file_contents = std::fs::read_to_string(&config.config_path)?;

    for line in file_contents.lines() {
        match line {
            "debug:" => {
                config.debug = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .to_lowercase()
                    .trim()
                    .eq("true");
            }
            "link:" => {
                config.link = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .to_lowercase()
                    .trim()
                    .eq("true");
            }
            "config_path:" => {
                config.config_path = line.split(":").nth(1).unwrap().trim().to_string();
            }
            "interval:" => {
                config.interval = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap();
            }
            "sleep:" => {
                config.sleep = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap();
            }
            "part_one_format:" => {
                config.part_one_format = line.split(":").nth(1).unwrap().trim().to_string();
            }
            "part_two_format:" => {
                config.part_two_format = line.split(":").nth(1).unwrap().trim().to_string();
            }
            _ => {}
        };
    }
    Ok(config)
}

pub fn create_default() -> Config {
    let config = Config::default();

    std::fs::create_dir_all(std::path::Path::new(&config.config_path).parent().unwrap()).unwrap();

    let mut file = std::fs::File::create(&config.config_path).unwrap();
    file.write_all(
        format!(
            "debug: {}\nlink: {}\nconfig_path: {}\ninterval: {}\nsleep: {}\npart_one_format: {}\npart_two_format: {}",
            config.debug,
            config.link,
            config.config_path,
            config.interval,
            config.sleep,
            config.part_one_format,
            config.part_two_format
        ).as_bytes()
    ).unwrap();
    config
}

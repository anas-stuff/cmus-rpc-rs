use std::collections::HashMap;

pub enum State {
    PLAYING,
    PAUSED,
    STOPPED,
}

pub struct Response {
    pub tags: HashMap<String, String>,
    pub state: State,
    pub file_path: String,
    pub duration: u32,
    pub position: u32,
}

impl State {
    pub fn to_string(&self) -> &str {
        match self {
            State::PLAYING => "playing",
            State::PAUSED => "paused",
            State::STOPPED => "stopped",
        }
    }
}

impl Response {
    pub fn new(cmus_response: String) -> Response {
        let mut tags: HashMap<String, String> = HashMap::new();
        let mut state: State = State::STOPPED;
        let mut file_path: &str = "Unknown";
        let mut duration: u32 = 0;
        let mut position: u32 = 0;

        for line in cmus_response.lines() {
            if line.starts_with("tag") {
                let (tag, value) = line.split_once(" ").unwrap().1.split_once(" ").unwrap();
                tags.insert(tag.to_string(), value.to_string());
            } else if line.starts_with("status") {
                let (_, value) = line.split_once(" ").unwrap();
                state = match value {
                    "playing" => State::PLAYING,
                    "paused" => State::PAUSED,
                    "stopped" => State::STOPPED,
                    _ => panic!("Unknown status: {}", value),
                };
            } else if line.starts_with("file") {
                file_path = line.split_once(" ").unwrap().1;
            } else if line.starts_with("duration") {
                duration = line.split_once(" ").unwrap().1.parse::<u32>().unwrap();
            } else if line.starts_with("position") {
                position = line.split_once(" ").unwrap().1.parse::<u32>().unwrap();
            }
        }

        Response {
            tags,
            state,
            file_path: file_path.to_string(),
            duration,
            position,
        }
    }

    pub fn title(&self) -> String {
        self.tags
            .get("title")
            .unwrap_or(&self.file_name())
            .to_string()
    }

    pub fn file_name(&self) -> String {
        std::path::Path::new(&self.file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

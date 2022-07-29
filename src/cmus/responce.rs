use std::collections::HashMap;

pub enum Tag {
    TITLE,
    ALBUM,
    ARTIST,
    TRACK_NUMBER,
    DISC_NUMBER,
    ALBUM_ARTIST,
    GENRE,
    DATE,
    LABEL,
    BPM,
}

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

impl Tag {
    pub fn to_string(&self) -> &str {
        match self {
            Tag::TITLE => "title",
            Tag::ALBUM => "album",
            Tag::ARTIST => "artist",
            Tag::TRACK_NUMBER => "tracknumber",
            Tag::DISC_NUMBER => "discnumber",
            Tag::ALBUM_ARTIST => "albumartist",
            Tag::GENRE => "genre",
            Tag::DATE => "date",
            Tag::LABEL => "label",
            Tag::BPM => "bpm",
        }
    }

    pub fn get_tag_from_string(tag: &str) -> Tag {
        match tag {
            "title" => Tag::TITLE,
            "album" => Tag::ALBUM,
            "artist" => Tag::ARTIST,
            "tracknumber" => Tag::TRACK_NUMBER,
            "discnumber" => Tag::DISC_NUMBER,
            "albumartist" => Tag::ALBUM_ARTIST,
            "genre" => Tag::GENRE,
            "date" => Tag::DATE,
            "label" => Tag::LABEL,
            "bpm" => Tag::BPM,
            _ => panic!("Unknown tag: {}", tag),
        }
    }
}

impl Response {
    pub fn new(cmus_response: String) -> Response {
        let mut tags: HashMap<String, String> = HashMap::new();
        let mut state: State = State::STOPPED;
        let mut file_path: &str;
        let mut duration: u32 = 0;
        let mut position: u32 = 0;

        for line in cmus_response.lines() {
            if line.starts_with("tag") {
                let (tag, value) = line.split_once(" ").unwrap();
                tags.insert(tag.to_string(), value.to_string());
            } else if line.starts_with("status") {
                let (_ , value) = line.split_once(" ").unwrap();
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
}
use crate::cmus;

pub fn format(format: &str, responce: &cmus::responce::Response) -> String {
    let mut formatted = format.to_string();
    formatted = formatted.replace("%title%", &responce.title());
    formatted = formatted.replace(
        "%artist%",
        &responce
            .tags
            .get("artist")
            .unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%album%",
        &responce.tags.get("album").unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%date",
        &responce.tags.get("date").unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%tracknumber%",
        &responce
            .tags
            .get("tracknumber")
            .unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%discnumber%",
        &responce
            .tags
            .get("discnumber")
            .unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%albumartist%",
        &responce
            .tags
            .get("albumartist")
            .unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%genre%",
        &responce.tags.get("genre").unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%label%",
        &responce.tags.get("label").unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "%bpm%",
        &responce.tags.get("bpm").unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace(
        "publisher",
        &responce
            .tags
            .get("publisher")
            .unwrap_or(&"Unknown".to_string()),
    );
    formatted = formatted.replace("%status%", &responce.state.to_string());
    formatted = formatted.replace("%file%", &responce.file_name());
    formatted = formatted.replace("%file path%", &responce.file_path);
    formatted = formatted.replace("%duration%", &responce.duration.to_string());
    formatted = formatted.replace("%position%", &responce.position.to_string());

    formatted
}

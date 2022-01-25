use regex::Regex;

/// Checkign files for get images when can display in browser
pub(crate) fn image_detector(filename: &str) -> bool {
    let ext_str = Regex::new(r"\w*$").unwrap().find(filename).unwrap().as_str();

    matches!(
        ext_str,
        "apng" | "avif" | "gif" |
        "jpg" | "jpeg" | "jpe" |
        "jif" | "jfif" | "png" |
        "svg" | "webp"
    )
}

use regex::Regex;

use crate::types::UUID;

/// Returns extension derived from filename
/// Extension with dot in ascii and lower case
pub(crate) fn ext_str(filename: &str) -> String {
    Regex::new(r"\.\w+$")
        .unwrap()
        .find(filename)
        .map(|m| m.as_str().to_ascii_lowercase())
        .unwrap_or_default()
}

/// Checkign files for get images when can display in browser
pub(crate) fn image_detector(filename: &str) -> bool {
    matches!(
        ext_str(filename).as_str(),
        ".apng" | ".avif" | ".gif" |
        ".jpg" | ".jpeg" | ".jpe" |
        ".jif" | ".jfif" | ".png" |
        ".svg" | ".webp"
    )
}

fn append_frag(text: &mut String, frag: &mut String) {
    if !frag.is_empty() {
        let encoded = frag.chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|ch| {
                u8::from_str_radix(&ch.iter().collect::<String>(), 16).unwrap()
            }).collect::<Vec<u8>>();
        text.push_str(&std::str::from_utf8(&encoded).unwrap());
        frag.clear();
    }
}


/// Function for mechanism to convert an URL-encoded string into its original unencoded form
pub(crate) fn url_decode(text: &str) -> String {
    let mut output = String::new();
    let mut encoded_ch = String::new();
    let mut iter = text.chars();
    while let Some(ch) = iter.next() {
        if ch == '%' {
            encoded_ch.push_str(&format!("{}{}", iter.next().unwrap(), iter.next().unwrap()));
        } else {
            append_frag(&mut output, &mut encoded_ch);
            output.push(ch);
        }
    }
    append_frag(&mut output, &mut encoded_ch);
    output
}

/// Prepares a username by removing leading special characters and decoding URL-encoded characters.
/// This function takes a raw username as input, removes start `#/@` characters, and then decodes URL-encoded characters.
pub(crate) fn prepare_username(raw_username: &str) -> String {
    url_decode(raw_username.trim_start_matches("#/@"))
}

/// Compares UUIDs wrapped in option
pub(crate) fn compare_op_uuid(first_uuid: &Option<UUID>, second_uuid: &Option<UUID>) -> bool {
    match (first_uuid, second_uuid) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(f_uuid), Some(s_uuid)) => f_uuid == s_uuid,
    }
}

/// Wraps a string in Option and returns None if it is empty.
pub(crate) fn wraps_text(text: String) -> Option<String> {
    match text.is_empty() {
        true => None,
        false => Some(text),
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn decode_1() {
        let input_test = "%D1%82%D0%B5%D1%81%D1%82";
        let output_test = String::from("тест");

        let result = url_decode(input_test);

        assert_eq!(output_test, result)
    }

    #[test]
    fn decode_2() {
        let input_test = "cadbase.rs/search?q=%60Abdu%27l-Bah%C3%A1";
        let output_test = String::from("cadbase.rs/search?q=`Abdu'l-Bahá");

        let result = url_decode(input_test);

        assert_eq!(output_test, result)
    }
}

use crate::types::{ShowFileInfo, DownloadFile};
use crate::services::LocaleKey;

// 1 kibibyte	1 KiB	2^10 = 1024 bytes
// 1 mebibyte	1 MiB	2^20 = 1048576 bytes
// 1 gibibyte	1 GiB	2^30 = 1073741824 bytes
// 1 tebibyte	1 TiB	2^40 = 1099511627776 bytes

pub trait Size {
    fn filesize(&self) -> usize;

    /// Makes the file size in a user friendly format (like "333.03 MB")
    fn show_size(&self) -> String {
        let (size, key) = match self.filesize() {
            // show bytes
            x @ 0..=999_usize =>
                return format!("{} {}", x, LocaleKey::Bytes.get_value()),
            // to kilobyte
            x @ 0..=999_999_usize => (x as f64 / 1e+3, LocaleKey::KB),
            // to megabyte
            x @ 0..=999_999_999_usize => (x as f64 / 1e+6, LocaleKey::MB),
            // to gigabyte
            x => (x as f64 / 1e+9, LocaleKey::GB),
            // to terabyte
            // x => (x as f64 / 1e+12, LocaleKey::TB),
        };
        format!("{:.2} {}", size, key.get_value())
    }
}

impl Size for ShowFileInfo {
    fn filesize(&self) -> usize {
        self.filesize
    }
}

impl Size for DownloadFile {
    fn filesize(&self) -> usize {
        self.filesize
    }
}

impl Size for f64 {
    fn filesize(&self) -> usize {
        *self as usize
    }
}
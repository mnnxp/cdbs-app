use crate::types::{ShowFileInfo, DownloadFile};
use crate::services::get_value_field;

// 1 kibibyte	1 KiB	2^10 = 1024 bytes
// 1 mebibyte	1 MiB	2^20 = 1048576 bytes
// 1 gibibyte	1 GiB	2^30 = 1073741824 bytes
// 1 tebibyte	1 TiB	2^40 = 1099511627776 bytes

pub trait Size {
    fn filesize(&self) -> usize;

    /// Makes the file size in a user friendly format (like "333.03 MB")
    fn show_size(&self) -> String {
        let (size, text_id) = match self.filesize() {
            // show bytes
            x @ 0..=999_usize =>
                return format!("{} {}", x, get_value_field(&316)),
            // to kilobyte
            x @ 0..=999_999_usize => (x as f64 / 1e+3, 317),
            // to megabyte
            x @ 0..=999_999_999_usize => (x as f64 / 1e+6, 318),
            // to gigabyte
            x => (x as f64 / 1e+9, 319),
            // to terabyte
            // x => (x as f64 / 1e+12, 320),
        };
        format!("{:.2} {}", size, get_value_field(&text_id))
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
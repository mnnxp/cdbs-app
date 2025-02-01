use yew::{Classes, classes};

/// Returns classes for table and sets the is-narrow class if element_count is greater than 5
pub fn get_classes_table(element_count: usize) -> Classes {
    match element_count > 5 {
        true => classes!("table", "is-fullwidth", "is-narrow"),
        false => classes!("table", "is-fullwidth"),
    }
}

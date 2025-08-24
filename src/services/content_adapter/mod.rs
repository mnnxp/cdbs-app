mod date_wrapper;
mod parsing_md;
mod type_access;

pub(crate) use date_wrapper::{two_dates_display, date_display};
pub(crate) use parsing_md::inner_markdown;

use chrono::NaiveDateTime;
use yew::Html;

pub(crate) trait ContentDisplay {
    /// Returns a name in converted form to display
    fn to_display(&self) -> Html;
}

pub(crate) trait Markdownable {
    /// Returns a result of converting a text as Markdown content into Html code
    fn to_markdown(&self) -> Html;

    /// Returns a result of converting a first line of text as Markdown content into Html code
    fn to_markdown_short(&self) -> Html;
}

impl Markdownable for String {
    /// Returns a VNode (Html) with the result of converting text to markdown style
    fn to_markdown(&self) -> Html {
        inner_markdown(&self)
    }

    /// Returns a VNode (Html) with the result of converting a first line of text to markdown style
    fn to_markdown_short(&self) -> Html {
        inner_markdown(&self.lines().next().unwrap_or_default())
    }
}

impl Markdownable for &str {
    /// Returns a VNode (Html) with the result of converting text to markdown style
    fn to_markdown(&self) -> Html {
        inner_markdown(self)
    }

    /// Returns a VNode (Html) with the result of converting a first line of text to markdown style
    fn to_markdown_short(&self) -> Html {
        inner_markdown(&self.lines().next().unwrap_or_default())
    }
}

pub(crate) trait DateDisplay {
    /// Returns VNode (Html) with convert dates to display.
    fn date_to_display(&self) -> Html;
}

impl DateDisplay for NaiveDateTime {
    /// Returns VNode (Html) with date information for displayed,
    /// adds date and time information in time tag.
    fn date_to_display(&self) -> Html {
        date_display(&self)
    }
}

pub(crate) trait ContactDisplay {
    /// Returns VNode (Html) with contact information
    fn contact_block(&self) -> Html;
}

pub(crate) trait SpecDisplay {
    /// Returns Html code with related directories and specifics
    fn spec_block(&self) -> Html;
}
use yew::{html, Callback, MouseEvent, Html, Classes, classes};
use crate::{services::{ext_str, image_detector}, types::Pathname};

pub fn res_btn(
  classes_icon: Classes,
  onclick: Callback<MouseEvent>,
  span_style: String,
  title: &str,
  pathname: Pathname,
) -> Html {
  html!{
    <a class={"button"} onclick={onclick} href={pathname.get_pathname()} title={title.to_string()}>
      <span class={"icon is-small"} style={span_style} >
        <i class={classes_icon}></i>
      </span>
    </a>
  }
}

pub fn res_fullwidth_btn(
  onclick: Callback<MouseEvent>,
  label: &str,
  pathname: Pathname,
) -> Html {
  html!{
    <a class={"button is-info is-light is-fullwidth has-text-weight-bold"} onclick={onclick} href={pathname.get_pathname()}>
      {label.to_string()}
    </a>
  }
}

pub fn res_file_btn(onclick: Callback<MouseEvent>, filename: String) -> Html {
  let mut style_color = "color: #767676;";
  let classes_icon = match ext_str(&filename).as_str() {
    ".txt" => classes!("far", "fa-file-alt"),
    ".pdf" => {
      style_color = "color: #e50707";
      classes!("far", "fa-file-pdf")
    },
    // OpenDocument Format is standardised by OASIS and adopted by ISO/IEC JTC1 SC34 (but are no icons)
    ".odt" | ".ods" | ".odp" => {
      style_color = "color: #43C330";
      classes!("far", "fa-file-alt")
    },
    ".docx" | ".doc" | ".rtf" => {
      style_color = "color: #0058a5";
      classes!("far", "fa-file-word")
    },
    ".xls" | ".xlsx" => {
      style_color = "color: #4aa500";
      classes!("far", "fa-file-excel")
    },
    ".ppt" | ".pptx" => {
      style_color = "color: #f78b67";
      classes!("far", "fa-file-powerpoint")
    },
    ".zip" | ".tar" | ".gz" | ".7z" | ".rar" => {
      style_color = "color: #1b1b1b";
      classes!("far", "fa-file-archive")
    },
    ".mp4" | ".mov" | ".flv" | ".avi" => {
      style_color = "color: #1872f0";
      classes!("far", "fa-file-video")
    },
    ".mp3" | ".ogg" | ".aac" | ".wav" => {
      style_color = "color: #1872f0";
      classes!("far", "fa-file-audio")
    },
    ".fcstd" | ".blend" | ".stl" | ".step" | ".stp" => {
      style_color = "color: #1872f0";
      classes!("far", "fa-file")
    },
    ".py" | ".sh" => classes!("far", "fa-file-code"),
    _ => {
      if image_detector(&filename) {
        style_color = "color: #1872f0";
        classes!("far", "fa-file-image")
      } else {
        classes!("far", "fa-file")
      }
    },
  };

  html!{
    <div class="button is-white" onclick={onclick}>
      <span class="icon">
        <i class={classes_icon} style={style_color}></i>
      </span>
      <span>{filename}</span>
    </div>
  }
}

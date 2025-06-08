use ratatui::style::{Style, Stylize};

use crate::theme::ItemStyle;

pub fn get_style_from_config(default_style: Option<Style>, item_style_config: ItemStyle) -> Style {
    let mut style = default_style.unwrap_or(Style::default());

    if let Some(fg) = item_style_config.fg {
        style = style.fg(fg);
    }
    if let Some(bg) = item_style_config.bg {
        style = style.bg(bg)
    }

    if let Some(bold) = item_style_config.bold {
        if bold {
            style = style.bold();
        }
    }
    if let Some(italic) = item_style_config.italic {
        if italic {
            style = style.italic();
        }
    }

    style
}

use ratatui::style::{Style, Stylize};

use crate::theme::ItemStyle;

pub fn get_style_from_config(item_style_config: ItemStyle) -> Style {
    let mut style = Style::default().fg(item_style_config.fg);
    if let Some(bg) = item_style_config.bg {
        style = style.bg(bg)
    };

    if item_style_config.bold {
        style = style.bold();
    }
    if item_style_config.italic {
        style = style.italic();
    }

    style
}

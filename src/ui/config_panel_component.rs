use crate::theme::theme_model::Theme;
use crate::ui::translate::Translate;
use crate::ui::utils::get_style_from_config;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

pub struct ConfigPanelComponent<'a> {
    pub translate: &'a Translate<'a>,
    pub theme: &'a Theme,
    pub theme_name: &'a str,
    pub focus: usize,
}

impl<'a> Widget for ConfigPanelComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", self.translate.config))
            .title_alignment(Alignment::Center);

        let inner_area = outer_block.inner(area);
        outer_block.render(area, buf);

        let padded_area = inner_area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        let normal_style = get_style_from_config(Some(Style::default()), self.theme.workday);
        let style = if self.focus == 0 {
            self.theme.get_panel_selected_style()
        } else {
            normal_style
        };

        Paragraph::new(format!("{}: {}", self.translate.theme, self.theme_name))
            .style(style)
            .render(padded_area, buf);
    }
}

use crate::theme::theme_model::Theme;
use crate::ui::translate::Translate;
use crate::ui::utils::get_style_from_config;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{
    Block, BorderType, Borders, List, ListItem, ListState, StatefulWidget, Widget,
};

pub struct ThemeSelectComponent<'a> {
    pub translate: &'a Translate<'a>,
    pub theme: &'a Theme,
    pub theme_names: &'a [&'a str],
    pub selected: usize,
}

impl<'a> Widget for ThemeSelectComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", self.translate.theme))
            .title_alignment(Alignment::Center);

        let inner_area = outer_block.inner(area);
        outer_block.render(area, buf);

        let padded_area = inner_area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });

        let items: Vec<ListItem> = self
            .theme_names
            .iter()
            .map(|name| ListItem::new(*name))
            .collect();

        let normal_style = get_style_from_config(Some(Style::default()), self.theme.workday);
        let list = List::new(items)
            .style(normal_style)
            .highlight_style(self.theme.get_panel_selected_style());

        let mut state = ListState::default().with_selected(Some(self.selected));
        StatefulWidget::render(list, padded_area, buf, &mut state);
    }
}

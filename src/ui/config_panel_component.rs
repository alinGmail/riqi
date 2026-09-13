use crate::theme::theme_model::Theme;
use crate::ui::translate::Translate;
use crate::ui::utils::get_style_from_config;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, List, ListItem, ListState, StatefulWidget, Widget,
};

pub struct ConfigPanelComponent<'a> {
    pub translate: &'a Translate<'a>,
    pub theme: &'a Theme,
    pub theme_name: &'a str,
    pub show_lunar: bool,
    pub show_holiday: bool,
    pub focus: usize,
}

impl<'a> ConfigPanelComponent<'a> {
    fn on_off(&self, value: bool) -> &'a str {
        if value {
            self.translate.on
        } else {
            self.translate.off
        }
    }

    fn option_line(label: &str, value: &str, width: usize) -> ListItem<'static> {
        let label_span = Span::raw(label.to_string());
        let value_span = Span::raw(value.to_string());
        let pad = width.saturating_sub(label_span.width() + value_span.width());
        ListItem::new(Line::from(vec![
            label_span,
            Span::raw(" ".repeat(pad)),
            value_span,
        ]))
    }
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

        let items = vec![
            Self::option_line(
                self.translate.theme,
                self.theme_name,
                padded_area.width as usize,
            ),
            Self::option_line(
                self.translate.lunar,
                self.on_off(self.show_lunar),
                padded_area.width as usize,
            ),
            Self::option_line(
                self.translate.holiday,
                self.on_off(self.show_holiday),
                padded_area.width as usize,
            ),
        ];

        let normal_style = get_style_from_config(Some(Style::default()), self.theme.workday);
        let list = List::new(items)
            .style(normal_style)
            .highlight_style(self.theme.get_panel_selected_style());

        let mut state = ListState::default().with_selected(Some(self.focus));
        StatefulWidget::render(list, padded_area, buf, &mut state);
    }
}

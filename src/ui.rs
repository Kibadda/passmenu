use crate::state::State;

use ratatui::{prelude::*, widgets::*};

pub fn ui(frame: &mut Frame, state: &mut State) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(0),
    ]);
    let [help_area, search_area, keys_area] = vertical.areas(frame.area());

    let text = Text::from(Line::from(vec![
        "Esc".bold(),
        " (exit), ".into(),
        "Enter".bold(),
        " (launch)".into(),
    ]))
    .patch_style(Style::default());
    let help = Paragraph::new(text);
    frame.render_widget(help, help_area);

    let search = Paragraph::new(state.input.as_str())
        .style(Style::default())
        .block(Block::bordered().title(" Search "));
    frame.render_widget(search, search_area);
    frame.set_cursor_position((
        search_area.x + state.input.len() as u16 + 1,
        search_area.y + 1,
    ));

    let keys: Vec<ListItem> = state
        .filtered_keys
        .iter()
        .map(|key| ListItem::new(Line::from(Span::raw(key))))
        .collect();

    let keys = List::new(keys)
        .block(Block::bordered().title(" Keys "))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightBlue),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(keys, keys_area, &mut state.list_state);
}

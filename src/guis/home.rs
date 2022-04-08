use iced::{Column, Text, Row, Button, Align};
use crate::Message;

pub fn render<'a>(gui: &'a mut crate::MusicGui, mut content: Column<'a, Message>) -> Column<'a, Message> {
    let text = Text::new("Navigation")
        .size(40).vertical_alignment(iced::VerticalAlignment::Top);
    content = content.push(text);
    for page in &mut gui.pages {
        let btn = Button::new(
            &mut page.nav_state,
            Text::new(format!("Go to {}", page.name))
        )
            // .padding(10)
            .on_press(Message::Navigate(page.state.clone()))
            .style(crate::style::Button::Primary);
        content = content.push(
            Row::new()
                .spacing(10)
                .align_items(Align::Center)
                .push(Text::new(page.name.as_str()))
                .push(
                    btn
                )
        );
    }
    content
}
use iced::{Text, Element};
use crate::Message;

pub fn render<'a>(_gui: &crate::MusicGui) -> Vec<Element<'a, Message>> {
    let text = Text::new("Placeholder")
        .size(40).vertical_alignment(iced::VerticalAlignment::Center);
    vec![text.into()]
}
use iced::{Text, Element};
use crate::Message;

pub struct HomePage;

impl HomePage {
    pub fn new() -> Self {
        Self
    }
    pub fn render<'a>(&self) -> Vec<Element<'a, Message>> {
        let text = Text::new("Placeholder")
            .size(40).vertical_alignment(iced::VerticalAlignment::Center);
        vec![text.into()]
    }
}
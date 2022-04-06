use iced::{
    executor, Application, Button, Column,
    Command, Container, Element, Length, Row, Settings, Text, Clipboard,
};

pub fn main() -> iced::Result {
    MusicGui::run(Settings::default())
}

struct MusicGui {
    state: State
}

enum State {
    NotLoading,
    Loading,
    Loaded
}

#[derive(Debug, Clone)]
enum Message {
    Load
}

impl Application for MusicGui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MusicGui, Command<Message>) {
        (
            MusicGui {
                state: State::NotLoading
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Youtube music")
    }

    fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Load => match self.state {
                State::NotLoading => {
                    self.state = State::Loading;
                }
                _ => panic!("Tried to load while already loading something!")
            }
        }

        Command::none()
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     match self.state {
    //         State::Idle => Subscription::none(),
    //         State::Ticking { .. } => {
    //             time::every(Duration::from_millis(10)).map(Message::Tick)
    //         }
    //     }
    // }

    fn view(&mut self) -> Element<Message> {
        let text = Text::new("Hello world")
        .size(40);

        let content = Column::new()
            .align_items(iced::Align::Center)
            .spacing(20)
            .push(text);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
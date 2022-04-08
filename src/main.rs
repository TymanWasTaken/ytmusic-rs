mod guis;

use std::sync::Arc;
use iced::{
    executor, Application, Column,
    Command, Container, Element, Length, Settings, Text, Clipboard, futures::lock::Mutex, VerticalAlignment, button,
};
use ytmusic_api::{structs::Playlist, YtMusicClient};

pub fn main() -> iced::Result {
    MusicGui::run(Settings::default())
}

pub struct Page {
    pub name: String,
    pub state: State,
    pub nav_state: button::State
}

#[allow(dead_code)]
pub struct MusicGui {
    state: State,
    pages: Vec<Page>,
    playlists: Option<Vec<Playlist>>,
    client: Arc<Mutex<YtMusicClient>>
}

#[derive(Debug, Clone)]
pub enum State {
    Home,
    Playlists
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadPlaylists(Vec<Playlist>),
    HeadersSet,
    Navigate(State)
}

impl Application for MusicGui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MusicGui, Command<Message>) {
        let client = Arc::new(Mutex::new(YtMusicClient::new("headers.txt")));
        let client_clone = client.clone();
        (
            MusicGui {
                state: State::Home,
                playlists: None,
                client: client.clone(),
                pages: vec![
                    Page {
                        name: "Home".to_string(),
                        state: State::Home,
                        nav_state: button::State::new()
                    },
                    Page {
                        name: "Playlists".to_string(),
                        state: State::Playlists,
                        nav_state: button::State::new()
                    }
                ]
            },
            Command::perform(async move {
                let client = client_clone.lock().await;
                client.set_headers().await;
            }, |_| {
                Message::HeadersSet
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Youtube music")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        let mut command = Command::none();
        match message {
            Message::HeadersSet => (),
            Message::LoadPlaylists(playlists) => {
                self.playlists = Some(playlists);
            },
            Message::Navigate(state) => {
                match &state {
                    State::Home => (),
                    State::Playlists => command = guis::library_playlists::load(self.client.clone()),
                }
                self.state = state;
            }
        }

        command
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     match self.state {
    //         State::Loading { .. } => {
    //             Subscription::from(async {

    //             })
    //         },
    //         _ => Subscription::none()
    //     }
    // }

    fn view(&mut self) -> Element<Message> {
        let text = Text::new("Youtube Music")
            .size(40).vertical_alignment(VerticalAlignment::Top);

        let content = Column::new()
            .align_items(iced::Align::Center)
            .spacing(20)
            .push(text);

        let content = match &self.state {
            State::Home => guis::home::render(self, content),
            State::Playlists => guis::library_playlists::render(self, content)
        };

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

    #[allow(dead_code)]
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
                border_radius: 5.0,
                shadow_offset: Vector::new(0.5, 0.5),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}